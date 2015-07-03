#version 140

uniform mat4 view_transform;

in vec3 vertex_position;

in mat4 model_transform;

in vec3 vertex_color;
out vec3 vColor;

void main() {
    gl_Position = view_transform * model_transform * vec4(vertex_position, 1.0);
    vColor = vertex_color;
}
