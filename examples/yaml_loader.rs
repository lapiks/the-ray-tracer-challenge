use std::{env, path::PathBuf};

use ray_tracer::{YamlLoader, World};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("No command argument found. Please specify a yaml file path");
    //     return
    // }
    // let scene_file = &args[1];
    let scene_file = "examples/yaml_scenes/shadow-glamour-shot.yml";

    let source = std::fs::read_to_string(scene_file).unwrap();
    let loader = YamlLoader::load_from_str(source.as_str());

    let world = World::new()
        .with_objects(loader.objects().to_owned())
        .with_lights(loader.lights().to_owned());

    let canvas = loader.camera().unwrap().render(&world, 5);
    let mut export_path = PathBuf::from(scene_file);
    export_path.set_extension("png");
    canvas.export(export_path).unwrap()
}
