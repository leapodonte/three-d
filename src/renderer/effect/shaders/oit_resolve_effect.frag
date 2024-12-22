
// shader outputs
layout (location = 0) out vec4 outColor;

// epsilon number
const float EPSILON = 0.00001f;

// calculate floating point numbers equality accurately
bool isApproximatelyEqual(float a, float b)
{
    return abs(a - b) <= (abs(a) < abs(b) ? abs(b) : abs(a)) * EPSILON;
}

// get the max value between three values
float max3(vec3 v)
{
    return max(max(v.x, v.y), v.z);
}

void main()
{
    ivec2 fragCoord = ivec2(gl_FragCoord.xy);
    vec4 accum = texelFetch(accumColorMap, fragCoord, 0);
    float a = 1.0 - accum.a;
    accum.a = texelFetch(accumAlphaMap, fragCoord, 0).r;
    outColor = vec4(a * accum.rgb / clamp(accum.a, 0.001, 50000.0), a);
    outColor.rgb = tone_mapping(outColor.rgb);
    outColor.rgb = color_mapping(outColor.rgb);
}
