extern crate git2;

use std::path::Path;
use git2::{ Repository, Cred, RemoteCallbacks, FetchOptions };
use git2::build::{ RepoBuilder };

fn main() {
	let repo_url = "git@github.com:jamwaffles/git2-rs-github-clone-demo.git";
	let repo_clone_path = "workspace/";

	println!("Cloning {} into {}", repo_url, repo_clone_path);

	let mut builder = RepoBuilder::new();
	let mut callbacks = RemoteCallbacks::new();
	let mut fetch_options = FetchOptions::new();

	callbacks.credentials(|_, _, _| {
		let credentials = 
			Cred::ssh_key(
				"git", 
				Some(Path::new("/Users/jwaples/.ssh/id_rsa.pub")), 
				Path::new("/Users/jwaples/.ssh/id_rsa"), 
				None
			).expect("Could not create credentials object");

		Ok(credentials)
	});

	fetch_options.remote_callbacks(callbacks);
	
	builder.fetch_options(fetch_options);

	let repo = builder.clone(repo_url, Path::new(repo_clone_path)).expect("Could not clone repo");

	println!("Clone complete");

	// Do things with `repo` here
}
