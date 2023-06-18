use crate::structs::Command;

pub mod leave;
pub mod play;
pub mod queue;
pub mod repeat;
pub mod skip;

pub fn commands() -> [Command; 5] {
	[
		play::command(),
		leave::command(),
		queue::command(),
		skip::command(),
		repeat::command(),
	]
}
