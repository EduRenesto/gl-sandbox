#version 430

in vec3 out_Normal;
out vec4 color;

void main() {
    color = vec4(out_Normal, 1.0);
}
