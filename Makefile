dev:
	cargo build
	cd frontend && bun run dev

build: 
	cargo build
	cd frontend && bun run build

release:
	cargo build --release
	cd frontend && bun run build
