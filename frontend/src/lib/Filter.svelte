<script lang="ts">
    export let name: string;
    export let values: { [key: string]: number[] };

    export let selected: number[][];

    let filter: string = "";
</script>

<fieldset>
    <legend>{name}</legend>
    <input type="text" bind:value={filter}/>
    <div class="list">
        {#each Object.entries(values) as [key, value]}
            {#if key.toLowerCase().includes(filter.toLowerCase())}
                <label>
                    <input type="checkbox" bind:group="{selected}" value={value}>
                    {key}
                </label>
            {/if}
        {/each}
    </div>
</fieldset>

<style>
    fieldset {
        border: 1px solid;
        padding: 10px;
        max-height: 200px; /* Adjust this value as needed */
        text-align: left;
    }

    .list {
        overflow-y: auto;
        max-height: 150px; /* Adjust this value as needed */
    }

    label {
        display: block;
        margin-bottom: 10px;
    }
</style>
