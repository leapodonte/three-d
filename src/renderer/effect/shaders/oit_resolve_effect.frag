
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
    // fragment coordinates
    ivec2 coords = ivec2(gl_FragCoord.xy);

    // fragment accumAlpha
    float accumAlpha = texelFetch(accumAlphaMap, coords, 0).r;

    // save the blending and color texture fetch cost if there is not a transparent fragment
    if (isApproximatelyEqual(accumAlpha, 1.0f))
        discard;

    // fragment color
    vec4 accumColor = texelFetch(accumColorMap, coords, 0);

    // suppress overflow
    if (isinf(max3(abs(accumColor.rgb))))
        accumColor.rgb = vec3(accumColor.a);

    // prevent floating point precision bug
    vec3 average_color = accumColor.rgb / max(accumColor.a, EPSILON);

    // blend pixels
    outColor = vec4(average_color, 1.0f - accumAlpha);
}
