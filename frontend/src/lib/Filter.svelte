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

    function clickOutside(node: HTMLElement, {enabled: initialEnabled, cb}: { enabled: boolean, cb: Function }) {
        const handleOutsideClick = ({target}: MouseEvent) => {
            if (target instanceof Node && !node.contains(target)) {
                cb();
            }
        };

        function update({enabled}: { enabled: boolean }) {
            if (enabled) {
                window.addEventListener('click', handleOutsideClick);
            } else {
                window.removeEventListener('click', handleOutsideClick);
            }
        }

        update({enabled: initialEnabled});
        return {
            update,
            destroy() {
                window.removeEventListener('click', handleOutsideClick);
            }
        };
    }

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

<div class={open ? 'filter open' : 'filter'} use:clickOutside={{ enabled: open, cb: () => open = false }}>
    <button class="filter-box" on:click={() => $open_filter === name ? $open_filter = "" : $open_filter = name}>
        <span class="filter-name">{name}</span>
        {#if count > 0}<span class="filter-count">{count}</span>{/if}
        <span class="filter-wedge">❮</span>
    </button>
    <div class={open ? 'filter-popup' : 'filter-popup hidden'}>

        {#if Object.entries(values).length > 10}
            <div class="filter-search">
                🔍 <input type="text" bind:value={filter} placeholder="Search { name }"/>
                <!--<button on:click={() => filter = ""}>𐄂</button>-->
            </div>
        {/if}


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

        {#if Object.entries(values).length > 5}
            <div class="filter-bottom">
                Sort by
                <button class={ sort_by === SortType.Name ? sort_direction ? "sort-item sorted asc" : 'sort-item sorted desc' : "sort-item"}
                        on:click={() => update_sort(SortType.Name)}>Name
                </button>
                <button class={ sort_by === SortType.Count ? sort_direction ? "sort-item sorted asc" : 'sort-item sorted desc' : "sort-item"}
                        on:click={() => update_sort(SortType.Count)}>Count
                </button>

                Select
                <button on:click={select_all}>🗹</button>
                <button on:click={select_none}>☐</button>
            </div>
        {/if}

    </div>
</div>
