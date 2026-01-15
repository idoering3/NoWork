<script>
    import { quartOut } from "svelte/easing";
    import { fly } from "svelte/transition";

</script>
<div style="padding: 3rem;">
    <h1 id="gradient-text" transition:fly={{ y: 30, delay: 150, duration: 1500, easing: quartOut}}>
        NoWork
    </h1>
    <p transition:fly={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
        NoWork was made by Ian Doering. It follows the MIT License.
        <br><br>
        Copyright © {(new Date()).getFullYear()} Ian Doering
        <br><br>
        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
        <br><br>
        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
        <br><br>
        THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
    </p>
</div>

<style>
    h1 {
        font-family: "Playfair Display Italic", sans-serif;
    }
    #gradient-text {
        font-weight: bold;
        display: inline-block;
        background: linear-gradient(270deg, var(--primary-dark), var(--highlight-color), var(--highlight-color));
        background-size: 600% 600%; /* bigger than text for smooth animation */
        -webkit-text-fill-color: transparent;
        animation: move-gradient 6s ease infinite;
        background-clip: text;
        left: calc(50% + 1rem) 
    }

    @keyframes move-gradient {
        0% { background-position: 0% 50%; }
        50% { background-position: 100% 50%; }
        100% { background-position: 0% 50%; }
    }
</style>