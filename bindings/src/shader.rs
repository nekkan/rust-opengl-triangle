use {
    crate::gl_bindings,
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
    fn new(vertex_shader: &str, fragment_shader: &str) -> Shader {
        let vertex_shader = CString::new(vertex_shader.as_bytes()).unwrap();
        let fragment_shader = CString::new(fragment_shader.as_bytes()).unwrap();
        let mut shader = Shader { id: 0 };

        unsafe {
            let vertex = compile_shader(&vertex_shader, gl_bindings::VERTEX_SHADER);
            check_for_compilation_errors(vertex, "VERTEX");

            let fragment = compile_shader(&fragment_shader, gl_bindings::FRAGMENT_SHADER);
            check_for_compilation_errors(fragment, "FRAGMENT");

            let id = gl_bindings::CreateProgram();
            gl_bindings::AttachShader(id, vertex);
            gl_bindings::AttachShader(id, fragment);
            gl_bindings::LinkProgram(id);
            check_for_compilation_errors(id, "PROGRAM");

            // delete the shaders as they're linked into our program and no longer necessary
            gl_bindings::DeleteShader(vertex);
            gl_bindings::DeleteShader(fragment);
            shader.id = id;
        };

        return shader;
    }
}

unsafe fn compile_shader(shader_code: &CString, shader_type: GLuint) -> GLuint {
    let shader = gl_bindings::CreateShader(shader_type);
    gl_bindings::ShaderSource(shader, shader_code.as_ptr(), ptr::null());
    gl_bindings::CompileShader(shader);
    return shader;
}

unsafe fn check_for_compilation_errors(shader: u32, shader_type: &str) {
    let mut success = gl_bindings::FALSE as GLint;
    let mut information = Vec::with_capacity(1024);
    information.set_len(1024 - 1); // subtract 1 to skip the trailing null character

    let status = if shader_type != "PROGRAM" { gl_bindings::COMPILE_STATUS } else { gl_bindings::LINK_STATUS };
    gl_bindings::GetShaderiv(shader, status, &mut success);

    gl_bindings::GetShaderInfoLog(shader, 1024, ptr::null_mut(), information.as_mut_ptr() as *mut GLchar);
    println!("Error of type {}:\n{}", shader_type, str::from_utf8(&information).unwrap());
}

