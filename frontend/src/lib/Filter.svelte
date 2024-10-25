<script lang="ts">

    import {open_filter} from '../store/FilterStore.svelte';

    export let name: string;
    export let values: { [key: string]: number[] };
    export let selected: number[][];

    type SortType = "Name" | "Count";
    const NAME: SortType = "Name";
    const COUNT: SortType = "Count";

    let open: boolean = false;
    let filter: string = "";
    let sort_by: SortType = NAME;
    let sort_direction: boolean = true;

    function sort(items: { [key: string]: number[] }, sort_by: SortType, sort_direction: boolean) {
        let checked = Object.entries(items).filter((i) => selected.includes(i[1]));
        let unchecked = Object.entries(items).filter((i) => !selected.includes(i[1]));

        if (sort_by == COUNT) {
            checked = checked.sort((a, b) => b[1].length - a[1].length);
            unchecked = unchecked.sort((a, b) => b[1].length - a[1].length);
        }

        if (!sort_direction) {
            checked = checked.reverse();
            unchecked = unchecked.reverse();
        }

        return checked.concat(unchecked);
    }

    function update_sort(type: SortType) {
        if (sort_by == type) {
            sort_direction = !sort_direction;
        } else {
            sort_by = type;
            sort_direction = true;
        }
    }

    function select_all() {
        selected = [...Object.values(values)];
    }

    function select_none() {
        selected = [];
    }

    $: sorted_values = sort(values, sort_by, sort_direction);
    $: count = selected.length;
    $: open = $open_filter == name;
</script>

<div class={open ? 'filter open' : 'filter'}>
    <button class="filter-box" on:click={() => $open_filter === name ? $open_filter = "" : $open_filter = name}>
        <span class="filter-name">{name}</span>
        {#if count > 0}<span class="filter-count">{count}</span>{/if}
        <span class="filter-wedge">❮</span>
    </button>
    <div class={open ? 'filter-popup' : 'filter-popup hidden'}>
        <div class="filter-top">
            <button class={ sort_by === NAME ? sort_direction ? "sort-item sorted asc" : 'sort-item sorted desc' : "sort-item"}
                    on:click={() => update_sort(NAME)}>Name
            </button>
            <button class={ sort_by === COUNT ? sort_direction ? "sort-item sorted asc" : 'sort-item sorted desc' : "sort-item"}
                    on:click={() => update_sort(COUNT)}>Count
            </button>

            <button on:click={select_all}>All</button>
            <button on:click={select_none}>None</button>
        </div>

        <div class="filter-list">
            {#each sorted_values as [key, value]}
                {#if key.toLowerCase().includes(filter.toLowerCase())}
                    <label>
                        <input type="checkbox" bind:group="{selected}" value={value}>
                        {key} ({value.length})
                    </label>
                {/if}
            {/each}
        </div>
        <div class="filter-search">
            🔍 <input type="text" bind:value={filter}/>
            <button on:click={() => filter = ""}>Clear</button>
        </div>
    </div>
</div>
