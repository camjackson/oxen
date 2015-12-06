#version 140

uniform mat4 view_transform;
uniform mat4 perspective_transform;

in vec3 vertex_position;

in mat4 model_transform;

in vec3 vertex_color;
out vec3 vColor;

void main() {
    gl_Position = perspective_transform * view_transform * model_transform * vec4(vertex_position, 1.0);
    vColor = vertex_color;
}
