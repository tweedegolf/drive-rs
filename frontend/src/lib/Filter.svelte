<script lang="ts">
    export let name: string;
    export let values: { [key: string]: number[] };
    export let selected: number[][];

    let filter: string = "";
    let open: boolean = false;
    let count: number = 0;

    let by_count: boolean;

    $: sorted_values = by_count ? Object.entries(values).sort((a, b) => b[1].length - a[1].length) : Object.entries(values);
</script>

<div class={open ? 'filter open' : 'filter'} on:click={() => open = !open}>
  <div class="filter-box">
    <span class="filter-name">{name}</span>
    <span class="filter-count">{count}</span>
    <span class="filter-wedge">❮</span>
  </div>
  <div class={open ? 'filter-popup' : 'filter-popup hidden'}></div>
</div>

<!--
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
</fieldset>-->
