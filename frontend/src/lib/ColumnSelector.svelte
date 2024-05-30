<script lang="ts">
    export let selected: string[];
    export let options: string[];

    let filter: string = "";
</script>

<fieldset>
    <legend>Columns</legend>
    <input type="text" bind:value={filter}/>
    <div class="list">
        {#each options as value}
            {#if value.toLowerCase().includes(filter.toLowerCase())}
                <label>
                    <input type="checkbox" bind:group="{selected}" value={value}>
                    {value}
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
