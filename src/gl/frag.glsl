#version 100

varying lowp vec2 texcoord;
uniform sampler2D tex;

void main() {
	gl_FragColor = texture2D(tex, texcoord);
	//gl_FragColor = vec4(1, 0, 0, 1);
}
