use {
    crate::gl_bindings,
    crate::gl_bindings::Gl,
    crate::gl_bindings::types::*,
    std::{
        ffi::{CStr, CString},
        ptr, str,
    },
};

pub struct Shader {
    pub id: u32
}

impl Shader {
    fn new(gl: &Gl, vertex_shader: &str, fragment_shader: &str) -> Shader {
        let vertex_shader = CString::new(vertex_shader.as_bytes()).unwrap();
        let fragment_shader = CString::new(fragment_shader.as_bytes()).unwrap();
        let mut shader = Shader { id: 0 };

        unsafe {
            let vertex = compile_shader(gl, &vertex_shader, gl_bindings::VERTEX_SHADER);
            check_for_compilation_errors(gl, vertex, "VERTEX");

            let fragment = compile_shader(gl, &fragment_shader, gl_bindings::FRAGMENT_SHADER);
            check_for_compilation_errors(gl, fragment, "FRAGMENT");

            let id = gl.CreateProgram();
            gl.AttachShader(id, vertex);
            gl.AttachShader(id, fragment);
            gl.LinkProgram(id);
            check_for_compilation_errors(gl, id, "PROGRAM");

            // delete the shaders as they're linked into our program and no longer necessary
            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);
            shader.id = id;
        };

        return shader;
    }
}

unsafe fn compile_shader(gl: &Gl, shader_code: &CString, shader_type: GLuint) -> GLuint {
    let shader = gl.CreateShader(shader_type);
    gl.ShaderSource(shader, 1, &shader_code.as_ptr(), ptr::null());
    gl.CompileShader(shader);
    return shader;
}

unsafe fn check_for_compilation_errors(gl: &Gl, shader: u32, shader_type: &str) {
    let mut success = gl_bindings::FALSE as GLint;
    let mut information = Vec::with_capacity(1024);
    information.set_len(1024 - 1); // subtract 1 to skip the trailing null character

    let status = if shader_type != "PROGRAM" { gl_bindings::COMPILE_STATUS } else { gl_bindings::LINK_STATUS };
    gl.GetShaderiv(shader, status, &mut success);

    gl.GetShaderInfoLog(shader, 1024, ptr::null_mut(), information.as_mut_ptr() as *mut GLchar);
    println!("Error of type {}:\n{}", shader_type, str::from_utf8(&information).unwrap());
}

