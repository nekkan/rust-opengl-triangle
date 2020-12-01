extern crate bindings;
extern crate glutin;

use std::{mem, ptr};

use bindings::{
    gl_bindings,
    gl_bindings::{ARRAY_BUFFER, STATIC_DRAW, types::*},
};

const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0,
    0.5, -0.5, 0.0,
    0.0, 0.5, 0.0
];

pub fn get_triangle_vao(gl: &gl_bindings::Gl) -> u32 {
    let (mut vbo, mut vao) = (0, 0);
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.GenBuffers(1, &mut vbo);
        gl.BindVertexArray(vao);
        gl.BindBuffer(ARRAY_BUFFER, vbo);

        gl.BufferData(
            ARRAY_BUFFER,
            (VERTICES.len() * mem::size_of::<f32>()) as GLsizeiptr,
            VERTICES.as_ptr() as *const GLvoid,
            STATIC_DRAW,
        );

        gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl_bindings::FLOAT, // data type
            gl_bindings::FALSE, // normalized (int-to-float conversion)
            (3 * mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
            ptr::null(), // offset of the first component
        );

        // unbind both vbo and vao
        gl.BindBuffer(ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }
    return vao;
}
