use crate::structs::Command;

pub mod help;
pub mod ping;
pub mod system;

pub fn commands() -> [Command; 3] {
	[ping::command(), system::command(), help::command()]
}
