use crate::{
	base_embed,
	constants::DOT,
	structs::{CommandResult, Context},
};
use serenity_feature_only::builder::CreateEmbed;
use sysinfo::{ProcessExt, System, SystemExt};

/// Shows some information about the system.
#[poise::command(
	slash_command,
	prefix_command,
	rename = "system",
	aliases("sys", "system_info", "sys_info")
)]
pub async fn command(ctx: Context<'_>) -> CommandResult {
	let mut sys = System::new_all();

	sys.refresh_memory();
	sys.refresh_system();

	if let Ok(pid) = sysinfo::get_current_pid() {
		if let Some(process) = sys.process(pid) {
			let process_info = format!(
				"> {DOT} Memory Usage: **{}**\n> {DOT} CPU Usage: **{}%**\n> {DOT} Woke up: **<t:{}:R>**",
				format_bytes(process.memory()),
				process.cpu_usage().round(),
				process.start_time()
			);

			ctx.send(
				poise::CreateReply::default().embed(
					base_embed(CreateEmbed::default())
						.title("System Info")
						.description(process_info)
						.thumbnail(ctx.author().avatar_url().unwrap()),
				),
			)
			.await?;
		}
	}

	Ok(())
}

fn format_bytes(bytes: u64) -> String {
	if bytes == 0 {
		return String::from("0 bytes");
	}

	let k: u64 = 1024;
	let sizes = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
	let mut i = 0;
	let mut num = bytes as f64;

	while num >= k as f64 && i < sizes.len() - 1 {
		num /= k as f64;
		i += 1;
	}

	format!("{} {}", num.round(), sizes[i])
}
