import * as SunCalc from 'suncalc';

export class ShaderRenderer {

    canvas: HTMLCanvasElement
    gl: WebGL2RenderingContext
    program: WebGLProgram
    buffer: WebGLBuffer
    uniforms: { [name: string]: WebGLUniformLocation }
    animationId: number | null = null
    startTime: number = 0
    innerColor: [number, number, number]
    outerColor: [number, number, number]
    mouseX: number = 0;
    mouseY: number = 0;

    // Interpolation
    currentCenter: [number, number] = [0.5, 0.2]
    targetCenter: [number, number] = [0.5, 0.2]
    lerpFactor: number = 0.02 // adjust for speed of interpolation

    constructor(
        canvas: HTMLCanvasElement,
        innerColor: [number, number, number] = [1.0, 0.9, 0.5],
        outerColor: [number, number, number] = [0.8, 0.3, 0.1]
    ) {
        this.canvas = canvas;

        window.addEventListener("mousemove", (e) => {
            const rect = this.canvas.getBoundingClientRect();
            this.mouseX = (e.clientX - rect.left) / rect.width;
            this.mouseY = (e.clientY - rect.top) / rect.height;
        });

        const gl = canvas.getContext("webgl2")!;
        this.gl = gl;

        this.innerColor = innerColor;
        this.outerColor = outerColor;

        // Vertex shader
        const vertexSrc = `
            attribute vec2 a_position;
            void main() {
                gl_Position = vec4(a_position, 0.0, 1.0);
            }
        `;

        // Fragment shader
        const fragmentSrc = `
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_time;
            uniform vec3 u_innerColor;
            uniform vec3 u_outerColor;
            uniform vec2 u_center;

            void main() {
                vec2 uv = gl_FragCoord.xy / u_resolution.xy;
                vec2 toCenter = uv - u_center;
                float dist = length(toCenter);
                float angle = atan(toCenter.y, toCenter.x);

                float L = 2.0 * 3.14159;
                float S = 1.0;
                float A = 0.02;

                float waveSum = 0.0;
                waveSum += sin(angle * (L / 1.000) + u_time *  0.90 * S) * A * 0.64;
                waveSum += sin(angle * (L / 1.153) + u_time *  1.15 * S) * A * 0.40;
                waveSum += sin(angle * (L / 1.622) + u_time * -0.75 * S) * A * 0.48;
                waveSum += sin(angle * (L / 1.871) + u_time *  0.65 * S) * A * 0.43;
                waveSum += sin(angle * (L / 2.013) + u_time * -1.05 * S) * A * 0.32;

                float modDist = dist + waveSum;

                float t = smoothstep(0.85, 1.2, modDist);
                float noise = (fract(sin(dot(uv.xy * u_resolution.xy, vec2(12.9898,78.233))) * 43758.5453) - 0.5) / 255.0;
                t += noise;

                vec3 color = mix(u_innerColor, u_outerColor, t);

                gl_FragColor = vec4(color, 1.0);
            }
        `;

        const vertexShader = this.compileShader(gl.VERTEX_SHADER, vertexSrc);
        const fragmentShader = this.compileShader(gl.FRAGMENT_SHADER, fragmentSrc);
        const program = gl.createProgram()!;
        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);
        if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
            throw new Error("Failed to link program: " + gl.getProgramInfoLog(program));
        }
        this.program = program;

        this.buffer = gl.createBuffer()!;
        gl.bindBuffer(gl.ARRAY_BUFFER, this.buffer);
        const vertices = new Float32Array([
            -1, -1, 1, -1, -1, 1,
            -1, 1, 1, -1, 1, 1
        ]);
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

        this.uniforms = {
            u_resolution: gl.getUniformLocation(program, "u_resolution")!,
            u_time: gl.getUniformLocation(program, "u_time")!,
            u_innerColor: gl.getUniformLocation(program, "u_innerColor")!,
            u_outerColor: gl.getUniformLocation(program, "u_outerColor")!,
            u_center: gl.getUniformLocation(program, "u_center")!,
        };
    }

    compileShader(type: number, source: string): WebGLShader {
        const shader = this.gl.createShader(type)!
        this.gl.shaderSource(shader, source)
        this.gl.compileShader(shader)
        if (!this.gl.getShaderParameter(shader, this.gl.COMPILE_STATUS)) {
            throw new Error("Shader compile failed: " + this.gl.getShaderInfoLog(shader))
        }
        return shader
    }

    start() {
        this.startTime = performance.now()
        const loop = (time: number) => {
            this.draw(time)
            this.animationId = requestAnimationFrame(loop)
        }
        this.animationId = requestAnimationFrame(loop)
    }

    draw(currentTime: number) {
        const gl = this.gl
        const width = this.canvas.clientWidth * window.devicePixelRatio
        const height = this.canvas.clientHeight * window.devicePixelRatio

        if (this.canvas.width !== width || this.canvas.height !== height) {
            this.canvas.width = width
            this.canvas.height = height
            gl.viewport(0, 0, width, height)
        }

        gl.clearColor(0, 0, 0, 1)
        gl.clear(gl.COLOR_BUFFER_BIT)

        gl.useProgram(this.program)

        // apply mouse offset
        const offsetAmount = 0.05 // tweak for strength of effect
        const offsetX = (this.mouseX - 0.5) * offsetAmount;
        const offsetY = (this.mouseY - 0.5) * offsetAmount;
        const centerX = this.currentCenter[0] + offsetX;
        const centerY = this.currentCenter[1] - offsetY;
        // interpolate currentCenter -> targetCenter

        this.currentCenter[0] += (this.targetCenter[0] - centerX) * this.lerpFactor;
        this.currentCenter[1] += (this.targetCenter[1] - centerY) * this.lerpFactor;


        gl.uniform2f(this.uniforms.u_center, this.currentCenter[0], this.currentCenter[1])
        gl.uniform2f(this.uniforms.u_resolution, width, height)
        gl.uniform1f(this.uniforms.u_time, (currentTime - this.startTime) / 1000)
        gl.uniform3fv(this.uniforms.u_innerColor, this.innerColor)
        gl.uniform3fv(this.uniforms.u_outerColor, this.outerColor)

        gl.bindBuffer(gl.ARRAY_BUFFER, this.buffer)
        const posLoc = gl.getAttribLocation(this.program, "a_position")
        gl.enableVertexAttribArray(posLoc)
        gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 0, 0)

        gl.drawArrays(gl.TRIANGLES, 0, 6)
    }


    stop() {
        if (this.animationId !== null) {
            cancelAnimationFrame(this.animationId)
            this.animationId = null
        }
    }

    updateColors(inner: [number, number, number], outer: [number, number, number]) {
        this.innerColor = inner
        this.outerColor = outer
    }

    startUpdatingSun(intervalMs: number = 60_000, radiusX = 0.3, radiusY = 0.125) {
        this.updateSunTarget(radiusX, radiusY); // initial update
        setInterval(() => {
            this.updateSunTarget(radiusX, radiusY);
        }, intervalMs);
    }

    async updateSunTarget(radiusX = 0.4, radiusY = 0.2) {
        try {
            const { x, y } = await getUserSunPosition(radiusX, radiusY)
            this.targetCenter = [x, y]
        } catch (err) {
            console.warn("Could not get sun position:", err)
        }
    }
}





// helper function
export function cssVarToRGBArray(varName: string): [number, number, number] {
    // Get computed value
    const style = getComputedStyle(document.documentElement)
    const value = style.getPropertyValue(varName).trim() // e.g. "#f4a261" or "rgb(244,162,97)"

    // Hex format
    if (value.startsWith("#")) {
        const hex = value.slice(1)
        const r = parseInt(hex.slice(0, 2), 16) / 255
        const g = parseInt(hex.slice(2, 4), 16) / 255
        const b = parseInt(hex.slice(4, 6), 16) / 255
        return [r, g, b]
    }

    // rgb(r,g,b) format
    const match = value.match(/(\d+),\s*(\d+),\s*(\d+)/)
    if (match) {
        const [, r, g, b] = match
        return [parseInt(r)/255, parseInt(g)/255, parseInt(b)/255]
    }

    // fallback white
    return [1, 1, 1]
}

export async function getUserSunPosition(
    radiusX: number = 0.4,
    radiusY: number = 0.2
): Promise<{ x: number; y: number }> {
    const position = await new Promise<GeolocationCoordinates>((resolve, reject) => {
        if (!navigator.geolocation) return reject("Geolocation not supported");
        navigator.geolocation.getCurrentPosition(
        (pos) => resolve(pos.coords),
        (err) => reject(err)
        );
    });

    const sunPos = SunCalc.getPosition(new Date(), position.latitude, position.longitude);
    const x = 0.5 - radiusX * Math.sin(sunPos.azimuth);
    const y = 0.09 - radiusY * Math.sin(sunPos.altitude);

    console.log(x, y);
    return { x, y };
}
