<script lang="ts">
    export let userName: string;

    function createRng(seed: number) {
        // Mulberry32
        return function () {
            var t = (seed += 0x6d2b79f5);
            t = Math.imul(t ^ (t >>> 15), t | 1);
            t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
            return (t ^ (t >>> 14)) >>> 0;
        };
    }

    function getColor(name: string): string {
        let x = 0;
        for (let index = 0; index < name.length; index++) {
            x += name.charCodeAt(index);
        }

        let rng = createRng(x + 187187); // Add offset to seed, because the colors with offset 0 are ugly

        x = rng() % 360;

        return `hsl(${x}, 80%, 40%)`;
    }

    $: color = getColor(userName);
</script>

<span style="background-color: {color};">
    {userName}
</span>

<style>
    span {
        min-width: var(--user-size);
        max-width: var(--user-size);
        height: var(--user-size);
        border-radius: 50%;
        user-select: none;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        font-size: 10px;
    }
</style>
