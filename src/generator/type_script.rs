use crate::uniforms::UniformType;

use super::common;

pub fn generate_ts_types_file(file_path: &std::path::PathBuf, output_folder: &std::path::PathBuf) {
    let file = std::fs::read_to_string(&file_path).unwrap();
    let uniforms = common::extract_uniforms(file);

    let mut output_file = String::new();
    output_file.push_str("// DO NOT EDIT THIS FILE\n");
    output_file.push_str("// This file is generated by glsl-types\n\n");

    // Define the types for each uniform at the top of the file
    for uniform in &uniforms {
        output_file.push_str(&format!(
            "type {} = {};\n",
            uniform.name,
            convert_uniform_to_ts(&uniform.uniform_type)
        ));
    }

    output_file.push_str("\n");

    let output_file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let output_type_name = common::capitalize_first_letter(output_file_name);

    // Export a type that contains all the uniforms
    output_file.push_str(&format!("export type {} = {{\n", output_type_name));
    output_file.push_str("    uniforms: {\n");
    for uniform in uniforms {
        output_file.push_str(&format!("        {}: {};\n", uniform.name, uniform.name));
    }
    output_file.push_str("    };\n");
    output_file.push_str("};\n");

    let output_file_path = output_folder.join(format!("{}.ts", output_file_name));
    std::fs::write(output_file_path, output_file).unwrap();
}

fn convert_uniform_to_ts(uniform: &UniformType) -> String {
    let result =  match uniform {
        UniformType::Sampler2d => "WebGLTexture",

        UniformType::Float => "number",
        UniformType::Vec2 => "[number, number]",
        UniformType::Vec3 => "[number, number, number]",
        UniformType::Vec4 => "[number, number, number, number]",

        UniformType::Int => "number",
        UniformType::Ivec2 => "[number, number]",
        UniformType::Ivec3 => "[number, number, number]",
        UniformType::Ivec4 => "[number, number, number, number]",

        UniformType::Uint => "number",
        UniformType::Uvec2 => "[number, number]",
        UniformType::Uvec3 => "[number, number, number]",
        UniformType::Uvec4 => "[number, number, number, number]",
        
        UniformType::Bool => "boolean",
        UniformType::Bvec2 => "[boolean, boolean]",
        UniformType::Bvec3 => "[boolean, boolean, boolean]",
        UniformType::Bvec4 => "[boolean, boolean, boolean, boolean]",

        UniformType::Mat2 => "[number, number, number, number]",
        UniformType::Mat3 => "[number, number, number, number, number, number, number, number, number]",
        UniformType::Mat4 => "[number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number]",
    };

    return result.to_string()
}
