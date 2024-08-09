<script lang="ts">

    import {open_filter} from '../store/FilterStore.svelte';

    export let name: string;
    export let values: { [key: string]: number[] };
    export let selected: number[][];

    enum SortType {
      Name,
      Count,
    }

    let open: boolean = false;
    let filter: string = "";
    let sort_by: SortType = SortType.Name;
    let sort_direction: boolean = true;

    function sort(items, sort_by: SortType, sort_direction: boolean) {

      let checked = Object.entries(items).filter((i) => selected.includes(i[1]));
      let unchecked = Object.entries(items).filter((i) => !selected.includes(i[1]));

      if (sort_by == SortType.Count) {
        checked = checked.sort((a, b) => b[1].length - a[1].length);
        unchecked = unchecked.sort((a, b) => b[1].length - a[1].length);
      }

      if (!sort_direction) {
        checked = checked.reverse();
        unchecked = unchecked.reverse();
      }

      let results = checked.concat(unchecked);

      return results;
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
  <div class="filter-box" on:click={() => $open_filter == name ? $open_filter = "" : $open_filter = name}>
    <span class="filter-name">{name}</span>
    {#if count> 0}<span class="filter-count">{count}</span>{/if}
    <span class="filter-wedge">❮</span>
  </div>
  <div class={open ? 'filter-popup' : 'filter-popup hidden'}>
    <div class="filter-top">
      <div class={ sort_by == SortType.Name ? sort_direction ? "sort-item sorted asc" : 'sort-item sorted desc' : "sort-item"} on:click={() => update_sort(SortType.Name)}>Name</div>
      <div class={ sort_by == SortType.Count ? sort_direction ? "sort-item sorted asc" : 'sort-item sorted desc' : "sort-item"} on:click={() => update_sort(SortType.Count)}>Count</div>

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
      🔍 <input type="text" bind:value={filter} />
      <button on:click={() => filter = ""}>Clear</button>
    </div>
  </div>
</div>
