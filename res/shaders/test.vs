#version 430

layout(location=0) in vec3 in_Position;
layout(location=1) in vec3 in_Normal;
layout(location=2) in vec2 un_TexCoord;

out vec3 out_Normal;

uniform mat4 viewMatrix;
uniform mat4 projMatrix;

void main() {
    out_Normal = in_Normal;
    gl_Position = (projMatrix * viewMatrix) * vec4(in_Position, 1.0);
}
