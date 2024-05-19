use crate::log::{self, print_level};

use super::common;
use colored::Colorize;
use glsl::syntax::TypeSpecifierNonArray;

pub fn generate_ts_types_file(
  vertex_file_path: &std::path::PathBuf,
  fragment_file_path: &std::path::PathBuf,
  output_folder: &std::path::PathBuf,
) -> bool {
  let vertex_file = std::fs::read_to_string(&vertex_file_path).unwrap();
  let fragment_file = std::fs::read_to_string(&fragment_file_path).unwrap();

  let vertex_data = common::extract_shader_data(&vertex_file, &vertex_file_path);
  let fragment_data = common::extract_shader_data(&fragment_file, &fragment_file_path);

  // We need to combine the uniforms from both the vertex and fragment shaders.
  // We need to check that if a uniform is defined in both shaders, the type is the same.
  // If the type is different, we should throw an error.

  for vertex_uniform in &vertex_data.uniforms {
    for fragment_uniform in &fragment_data.uniforms {
      if vertex_uniform.name == fragment_uniform.name
        && vertex_uniform.uniform_type != fragment_uniform.uniform_type
      {
        print_level(log::Level::ERROR);
        println!(
          "Uniform {} is defined with different types in the vertex and fragment shaders",
          vertex_uniform.name.bright_red().bold()
        );

        return false;
      }
    }
  }

  // Combine the uniforms from both shaders. Avoid duplicates.
  let mut uniforms: Vec<common::Uniform> = vertex_data.uniforms.clone();

  for uniform in fragment_data.uniforms.clone() {
    let mut found = false;
    for existing_uniform in &uniforms {
      if existing_uniform.name == uniform.name {
        found = true;
        break;
      }
    }

    if !found {
      uniforms.push(uniform);
    }
  }

  // Varyings should be defined in both shaders
  for vertex_varying in &vertex_data.varyings {
    let mut found = false;
    for fragment_varying in &fragment_data.varyings {
      if vertex_varying.name == fragment_varying.name {
        found = true;
        break;
      }
    }

    if !found {
      print_level(log::Level::ERROR);
      println!(
        "Varying {} is defined in the vertex shader but not in the fragment shader",
        vertex_varying.name.bright_red().bold()
      );
      return false;
    }
  }

  for fragment_varying in &fragment_data.varyings {
    let mut found = false;
    for vertex_varying in &vertex_data.varyings {
      if fragment_varying.name == vertex_varying.name {
        found = true;
        break;
      }
    }

    if !found {
      print_level(log::Level::ERROR);
      println!(
        "Varying {} is defined in the fragment shader but not in the vertex shader",
        fragment_varying.name.as_str().bright_red().bold()
      );
      return false;
    }
  }

  // Varyings should be the same in both shaders
  for vertex_varying in &vertex_data.varyings {
    for fragment_varying in &fragment_data.varyings {
      if vertex_varying.name == fragment_varying.name
        && vertex_varying.varying_type != fragment_varying.varying_type
      {
        print_level(log::Level::ERROR);
        println!(
          "Varying {} is defined with different types in the vertex and fragment shaders",
          vertex_varying.name.as_str().bright_red().bold()
        );

        return false;
      }
    }
  }

  let mut output_file = String::new();
  output_file.push_str("// DO NOT EDIT THIS FILE\n");
  output_file.push_str("// This file is generated by glsl-types\n\n");

  output_file.push_str(&format!(
    "const VERTEX_SHADER_SOURCE = `{}`;\n\n",
    &vertex_file
  ));
  output_file.push_str(&format!(
    "const FRAGMENT_SHADER_SOURCE = `{}`;\n\n",
    &fragment_file
  ));

  let output_file_name = vertex_file_path.file_stem().unwrap().to_str().unwrap();
  let output_type_name = common::capitalize_first_letter(output_file_name);

  // Export a type that contains all the uniforms
  output_file.push_str(&format!("export const {} = {{\n", output_type_name));
  output_file.push_str("    uniforms: {\n");
  for uniform in &uniforms {
    output_file.push_str(&format!(
      "        {}: \"{}\",\n",
      &uniform.name,
      convert_glsl_to_ts_label(&uniform.uniform_type)
    ));
  }
  output_file.push_str("    },\n");
  output_file.push_str("    attributes: {\n");
  for attribute in &vertex_data.attributes {
    output_file.push_str(&format!(
      "        {}: \"{}\",\n",
      &attribute.name,
      convert_glsl_to_ts_label(&attribute.attribute_type)
    ));
  }
  output_file.push_str("    },\n");
  output_file.push_str("    vertexShaderSource: VERTEX_SHADER_SOURCE,\n");
  output_file.push_str("    fragmentShaderSource: FRAGMENT_SHADER_SOURCE,\n");
  output_file.push_str("};\n");

  let output_file_path = output_folder.join(format!("{}.ts", output_file_name));
  std::fs::write(output_file_path, output_file).unwrap();

  return true;
}

fn convert_glsl_to_ts_label(uniform: &TypeSpecifierNonArray) -> String {
  let result = match uniform {
    TypeSpecifierNonArray::Float => "float",
    TypeSpecifierNonArray::Vec2 => "vec2",
    TypeSpecifierNonArray::Vec3 => "vec3",
    TypeSpecifierNonArray::Vec4 => "vec4",

    TypeSpecifierNonArray::Int => "int",
    TypeSpecifierNonArray::IVec2 => "ivec2",
    TypeSpecifierNonArray::IVec3 => "ivec3",
    TypeSpecifierNonArray::IVec4 => "ivec4",

    TypeSpecifierNonArray::UInt => "uint",
    TypeSpecifierNonArray::UVec2 => "uvec2",
    TypeSpecifierNonArray::UVec3 => "uvec3",
    TypeSpecifierNonArray::UVec4 => "uvec4",

    TypeSpecifierNonArray::Bool => "bool",
    TypeSpecifierNonArray::BVec2 => "bvec2",
    TypeSpecifierNonArray::BVec3 => "bvec3",
    TypeSpecifierNonArray::BVec4 => "bvec4",

    TypeSpecifierNonArray::Mat2 => "mat2",
    TypeSpecifierNonArray::Mat3 => "mat3",
    TypeSpecifierNonArray::Mat4 => "mat4",
    _ => "UNKNOWN",
  };

  return result.to_string();
}
