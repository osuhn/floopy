use crate::structs::Command;

pub mod leave;
pub mod play;
pub mod queue;
pub mod repeat;
pub mod skip;
pub mod volume;

pub fn commands() -> [Command; 6] {
	[
		play::command(),
		leave::command(),
		queue::command(),
		skip::command(),
		repeat::command(),
		volume::command(),
	]
}
