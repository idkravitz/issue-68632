# The setup code for [issue 68632](https://github.com/rust-lang/rust/issues/68632)

Uncomment these lines in Cargo.toml to see that issue gone

	#[profile.release]
	#incremental=false
	#codegen-units=1


	#[profile.bench]
	#incremental=false
	#codegen-units=1

Comment them again to see that issue reproduces.

Thx all, you're magestic sausages!
