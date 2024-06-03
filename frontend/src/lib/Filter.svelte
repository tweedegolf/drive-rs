<script lang="ts">
    export let name: string;
    export let values: { [key: string]: number[] };

    export let selected: number[][];

    let filter: string = "";

    let by_count: boolean;

    $: sorted_values = by_count ? Object.entries(values).sort((a, b) => b[1].length - a[1].length) : Object.entries(values);
</script>

<fieldset>
    <legend>{name}</legend>
    <div>
        <input type="text" bind:value={filter} placeholder="Filter"/>
        <label><input type="checkbox" bind:checked={by_count}> Sort by count</label>
    </div>
    <div class="list">
        {#each sorted_values as [key, value]}
            {#if key.toLowerCase().includes(filter.toLowerCase())}
                <label>
                    <input type="checkbox" bind:group="{selected}" value={value}>
                    {key} ({value.length})
                </label>
            {/if}
        {/each}
    </div>
</fieldset>
