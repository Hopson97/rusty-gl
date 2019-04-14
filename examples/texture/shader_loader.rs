use gl;
use gl::types::*;
use rgl;
use std::io::Read;

use std::fs::File;


/*
 * Note: `GLShader` is just `pub struct(GLuint)`. This is to enforce correct type is passed into the functions
 */

pub fn load_shader(vertex_shader: String, fragment_shader: String) -> rgl::Program {
    let vertex_source = get_source(vertex_shader);
    let fragment_source = get_source(fragment_shader);

    let v_shader = compile_shader(vertex_source, rgl::ShaderType::Vertex);
    let f_shader = compile_shader(fragment_source, rgl::ShaderType::Fragment);
    let program = rgl::create_program();
    rgl::attach_shader(program, v_shader);
    rgl::attach_shader(program, f_shader);
    rgl::link_program(program);

    //Program linked, shaders no longer needed
    rgl::delete_shader(v_shader);
    rgl::delete_shader(f_shader);

    //Program object
    program
}

fn get_source(source_file: String) -> String {
    let mut file = File::open(source_file).expect("Unable to open shader file");

    let mut f_cont = String::new();
    file.read_to_string(&mut f_cont)
        .expect("Unable to read file");
    f_cont
}

fn compile_shader(source: String, ty: rgl::ShaderType) -> rgl::Shader {
    let shader = rgl::create_shader(ty);
    rgl::shader_source(shader, &source);
    rgl::compile_shader(shader);

    let mut compile_status = gl::FALSE as GLint;
    rgl::get_shader_iv(
        shader,
        rgl::ShaderInfoParam::CompileStatus,
        &mut compile_status,
    );

    if compile_status != (gl::TRUE as GLint) {
        let mut length = 0;
        rgl::get_shader_iv(shader, rgl::ShaderInfoParam::InfoLogLength, &mut length);

        let mut log_buffer = Vec::with_capacity(length as usize);
        unsafe { log_buffer.set_len((length as usize) - 1) };
        let mut len = 0;
        rgl::get_shader_info_log(shader, length, &mut len, &mut log_buffer);
        panic!("Could not compile shader");
    }
    shader
}
