mod init;
mod raytracer;
mod structures;

use init::initialize;

fn main() {
    let execution_context = initialize();
    raytracer::render(execution_context.scene, &execution_context.output);
}
