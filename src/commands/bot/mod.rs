use crate::structs::Command;

pub mod help;
pub mod ping;
pub mod servers;
pub mod system;

pub fn commands() -> [Command; 4] {
	[
		ping::command(),
		system::command(),
		help::command(),
		servers::command(),
	]
}
