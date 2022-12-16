use crate::core::MOVE;
use crate::model::goal::DatabaseGoal;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::move_store::MoveStore;
use crate::searcher::MovePruner;

#[derive(Default)]
pub struct PatternDatabaseIndexer;

#[derive(Clone, Copy)]
pub struct Node {
    cube: RubiksCubeIndexModel,
    r#move: Option<MOVE>,
    depth: u8,
}

impl PatternDatabaseIndexer {
    pub fn find_goal(
        goal: &mut impl DatabaseGoal,
        cube: &RubiksCubeIndexModel,
        move_store: &dyn MoveStore,
    ) {
        let timer = std::time::Instant::now();
        let mut node_stack = Vec::new();
        let mut ind_count = 0;
        let mut cur_depth = 0;
        let mut cur_node = Node {
            cube: Default::default(),
            r#move: None,
            depth: 0,
        };
        let num_moves = move_store.get_num_moves();
        goal.index(cube, 0);
        ind_count += 1;

        while !goal.is_satisfied(cube) {
            if node_stack.is_empty() {
                println!(
                    "Indexer: Finished depth {cur_depth}. Elapsed time {}s. Indexed {ind_count} states",
                    timer.elapsed().as_secs()
                );
                cur_depth += 1;

                node_stack.push(Node {
                    cube: cube.clone(),
                    r#move: None,
                    depth: 0,
                });
            }

            cur_node = *node_stack.last().unwrap();
            node_stack.pop();

            for i in 0..num_moves {
                let r#move = move_store.get_move(i);

                if cur_node.depth == 0 || !MovePruner::prune(r#move, cur_node.r#move.unwrap()) {
                    let mut cube_copy = cur_node.cube.clone();
                    let cube_copy_depth = cur_node.depth + 1;

                    cube_copy.r#move(r#move);

                    let db_ind = goal.get_database_index(&cube_copy);
                    if goal.get_num_moves_by_index(db_ind) < cube_copy_depth {
                        continue;
                    }

                    if cube_copy_depth == cur_depth {
                        if goal.index_by_id(db_ind, cube_copy_depth) {
                            ind_count += 1;
                        }
                    } else {
                        node_stack.push(Node {
                            cube: cube_copy,
                            r#move: Some(r#move),
                            depth: cube_copy_depth,
                        })
                    }
                }
            }
        }
    }
}
