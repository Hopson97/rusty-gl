use gl;
use gl::types::*;
use rusty_gl::shaders::*;
use rusty_gl::enums::*;
use std::str;
use std::io::Read;

use std::fs::File;

pub fn load_shader(vertex_shader: String, fragment_shader: String) -> GLuint {
    let vertex_source = get_source(vertex_shader);
    let fragment_source = get_source(fragment_shader);

    let v_shader = compile_shader(vertex_source, GLShaderType::Vertex);
    let f_shader = compile_shader(fragment_source, GLShaderType::Fragment);
    let program;
    unsafe{
        program = gl::CreateProgram();
    }
    gl_attach_shader(program, v_shader);
    gl_attach_shader(program, f_shader);
    gl_link_program(program);

    program
}

fn get_source(source_file: String) -> String {
    let mut file = File::open(source_file)
        .expect("Unable to open shader file");

    let mut f_cont = String::new();
    file.read_to_string(&mut f_cont).expect("Unable to read file");
    f_cont
}


fn compile_shader(source: String, ty: GLShaderType) -> GLuint {
    let shader = gl_create_shader(ty);
    gl_shader_source(shader, &source);
    gl_compile_shader(shader);
    
    let mut compile_status = gl::FALSE as GLint;
    gl_get_shader_iv(shader, GLShaderInfoParam::CompileStatus, &mut compile_status);

    if compile_status != (gl::TRUE as GLint) {
        let mut length = 0;
        gl_get_shader_iv(shader, GLShaderInfoParam::InfoLogLength, &mut length);

        let mut log_buffer = Vec::with_capacity(length as usize);
        unsafe{log_buffer.set_len((length as usize) - 1)};
        let mut len = 0;
        gl_get_shader_info_log(shader, length, &mut len, &mut log_buffer);
        panic!("Could not compile shader");
    }
    shader
}