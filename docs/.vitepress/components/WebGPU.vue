<template>
    <div id="webgpu-container">
        <div v-if="showAlert" style="color: #353535;margin-top: 20px;">
            <div style="line-height: 40px;">此浏览器版本不支持 WebGPU</div>
            <div style="font-size: 16px;color: #999999;">请使用 Chrome/Microsoft Edge 113 及以上版本，或者 Chrome/Edge Canary, FireFox
                Nightly 并
                <span><a href="https://jinleili.github.io/learn-wgpu-zh/#如何开启浏览器-webgpu-试验功能" class="a">开启 WebGPU
                        实验功能</a></span>
            </div>
        </div>
        <div v-if="loading">
            正在加载 WASM 模块 ...
        </div>
    </div>
</template>

<script>
export default {
    name: "web-gpu",
    props: {
        autoLoad: true,
    },
    data() {
        return {
            error: "",
            loading: true,
            showAlert: false,
        };
    },
    methods: {
        detectWebGPUThenLoad() {
            if ('navigator' in window && 'gpu' in navigator) {
                navigator.gpu.requestAdapter().then(adapter => {
                    // 浏览器支持 WebGPU
                    this.loadExample();
                }).catch(error => {
                    this.showAlert = true;
                });
            } else {
                // 浏览器不支持 navigator.gpu
                this.showAlert = true;
            }
        },
        async loadExample() {
            this.loading = true;
            try {
                const module = await import(/* @vite-ignore */`./wasm/webgpu/webgl-vs-webgpu.js`.replace('_', '-'));
                module.default().then((instance) => {
                    this.loading = false;
                }, (e) => {
                    if (!`${e}`.includes("don't mind me. This isn't actually an error!")) {
                        this.showErr(e);
                    } else {
                        this.loading = false;
                    }
                });

            } catch (e) {
                this.showErr(e);
            }
        },

        showErr(err) {
            this.error = `An error occurred loading "${this.example}": ${err}`;
            console.error(err);
            this.loading = false;
            this.showAlert = true;
        }
    },
    async mounted() {
        this.detectWebGPUThenLoad();
    }
};
</script>