pub mod rgl {
    use ::enums::*;

    use gl;
    use gl::types::*;
    use std::ffi::CString;
    use std::ptr;


    #[derive(Clone, Copy)]
    pub struct GLShader(GLuint);

    #[derive(Clone, Copy)]
    pub struct GLProgram(GLuint);

    #[derive(Clone, Copy)]
    pub struct GLUniformLocation(GLint);

    pub fn create_program() -> GLProgram {
        unsafe { GLProgram(gl::CreateProgram()) }
    }

    pub fn create_shader(type_: rgl::ShaderType) -> GLShader {
        unsafe { GLShader(gl::CreateShader(type_ as GLenum)) }
    }

    /// TODO: MISSING TWO PARAMS (to do with count)
    pub fn shader_source(shader: GLShader, source: &str) {
        unsafe {
            gl::ShaderSource(
                shader.0,
                1,
                &CString::new(source).unwrap().as_ptr(),
                ptr::null(),
            );
        }
    }

    pub fn compile_shader(shader: GLShader) {
        unsafe {
            gl::CompileShader(shader.0);
        }
    }

    pub fn get_shader_iv(shader: GLShader, parameter: rgl::ShaderInfoParam, status: &mut GLint) {
        unsafe {
            gl::GetShaderiv(shader.0, parameter as GLenum, status);
        }
    }

    pub fn get_shader_info_log(
        shader: GLShader,
        buffer_size: GLsizei,
        length: &mut GLsizei,
        info_log: &mut Vec<GLchar>,
    ) {
        unsafe {
            gl::GetShaderInfoLog(
                shader.0,
                buffer_size,
                length,
                info_log.as_mut_ptr() as *mut GLchar,
            );
        }
    }

    pub fn attach_shader(program: GLProgram, shader: GLShader) {
        unsafe {
            gl::AttachShader(program.0, shader.0);
        }
    }

    pub fn link_program(program: GLProgram) {
        unsafe {
            gl::LinkProgram(program.0);
        }
    }

    pub fn use_program(program: GLProgram) {
        unsafe {
            gl::UseProgram(program.0);
        }
    }

    pub fn delete_program(program: GLProgram) {
        unsafe {
            gl::DeleteProgram(program.0);
        }
    }

    pub fn delete_shader(shader: GLShader) {
        unsafe {
            gl::DeleteShader(shader.0);
        }
    }


    ///Shader uniforms


    pub fn get_uniform_location(program: GLProgram, name: &str) -> GLUniformLocation {
        unsafe {
            GLUniformLocation(gl::GetUniformLocation(program.0, CString::new(name).unwrap().as_ptr()))
        }
    }


    //Only going to use commonly used ones for now, may add the rest later
    pub fn uniform1f(location: GLUniformLocation, v0: GLfloat) {
        unsafe {
            gl::Uniform1f(location.0, v0);
        }
    }

    pub fn uniform2f(location: GLUniformLocation, v0: GLfloat, v1: GLfloat) {
        unsafe {
            gl::Uniform2f(location.0, v0, v1);
        }
    }

    pub fn uniform3f(location: GLUniformLocation, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
        unsafe {
            gl::Uniform3f(location.0, v0, v1, v2);
        }
    }

    pub fn uniform4f(location: GLUniformLocation, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
        unsafe {
            gl::Uniform4f(location.0, v0, v1, v2, v3);
        }
    }

    pub fn uniform1i(location: GLUniformLocation, v0: GLint) {
        unsafe {
            gl::Uniform1i(location.0, v0);
        }
    }

    pub fn uniform2i(location: GLUniformLocation, v0: GLint, v1: GLint) {
        unsafe {
            gl::Uniform2i(location.0, v0, v1);
        }
    }

    pub fn uniform3i(location: GLUniformLocation, v0: GLint, v1: GLint, v2: GLint) {
        unsafe {
            gl::Uniform3i(location.0, v0, v1, v2);
        }
    }

    pub fn uniform4i(location: GLUniformLocation, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
        unsafe {
            gl::Uniform4i(location.0, v0, v1, v2, v3);
        }
    }

    pub fn uniform1ui(location: GLUniformLocation, v0: GLuint) {
        unsafe {
            gl::Uniform1ui(location.0, v0);
        }
    }

    pub fn uniform2ui(location: GLUniformLocation, v0: GLuint, v1: GLuint) {
        unsafe {
            gl::Uniform2ui(location.0, v0, v1);
        }
    }

    pub fn uniform3ui(location: GLUniformLocation, v0: GLuint, v1: GLuint, v2: GLuint) {
        unsafe {
            gl::Uniform3ui(location.0, v0, v1, v2);
        }
    }

    pub fn uniform4ui(location: GLUniformLocation, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
        unsafe {
            gl::Uniform4ui(location.0, v0, v1, v2, v3);
        }
    }


    pub fn uniform1fv(location: GLUniformLocation, count: GLsizei, value: *const GLfloat) {
        unsafe {
            gl::Uniform1fv(location.0, count, value);
        }
    }

    pub fn uniform2fv(location: GLUniformLocation, count: GLsizei, value: *const GLfloat) {
        unsafe {
            gl::Uniform2fv(location.0, count, value);
        }
    }

    pub fn uniform3fv(location: GLUniformLocation, count: GLsizei, value: *const GLfloat) {
        unsafe {
            gl::Uniform3fv(location.0, count, value);
        }
    }

    pub fn uniform4fv(location: GLUniformLocation, count: GLsizei, value: *const GLfloat) {
        unsafe {
            gl::Uniform4fv(location.0, count, value);
        }
    }

    pub fn uniform1iv(location: GLUniformLocation, count: GLsizei, value: *const GLint) {
        unsafe {
            gl::Uniform1iv(location.0, count, value);
        }
    }

    pub fn uniform2iv(location: GLUniformLocation, count: GLsizei, value: *const GLint) {
        unsafe {
            gl::Uniform2iv(location.0, count, value);
        }
    }

    pub fn uniform3iv(location: GLUniformLocation, count: GLsizei, value: *const GLint) {
        unsafe {
            gl::Uniform3iv(location.0, count, value);
        }
    }

    pub fn uniform4iv(location: GLUniformLocation, count: GLsizei, value: *const GLint) {
        unsafe {
            gl::Uniform4iv(location.0, count, value);
        }
    }

    pub fn uniform1uiv(location: GLUniformLocation, count: GLsizei, value: *const GLuint) {
        unsafe {
            gl::Uniform1uiv(location.0, count, value);
        }
    }

    pub fn uniform2uiv(location: GLUniformLocation, count: GLsizei, value: *const GLuint) {
        unsafe {
            gl::Uniform2uiv(location.0, count, value);
        }
    }

    pub fn uniform3uiv(location: GLUniformLocation, count: GLsizei, value: *const GLuint) {
        unsafe {
            gl::Uniform3uiv(location.0, count, value);
        }
    }

    pub fn uniform4uiv(location: GLUniformLocation, count: GLsizei, value: *const GLuint) {
        unsafe {
            gl::Uniform4uiv(location.0, count, value);
        }
    }



    pub fn uniform_matrix_2fv(location: GLUniformLocation, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        unsafe {
            gl::UniformMatrix2fv(location.0, count, transpose, value);
        }
    }

    pub fn uniform_matrix_3fv(location: GLUniformLocation, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        unsafe {
            gl::UniformMatrix2fv(location.0, count, transpose, value);
        }
    }

    pub fn uniform_matrix_4fv(location: GLUniformLocation, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
        unsafe {
            gl::UniformMatrix2fv(location.0, count, transpose, value);
        }
    }
}