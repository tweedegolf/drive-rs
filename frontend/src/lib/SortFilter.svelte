<script lang="ts">

    import {open_filter, sort_by} from '../store/FilterStore.svelte';

    export let name: string;

    let open: boolean = false;

    function update_sort(type: string) {
      $sort_by = type;
    }

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

    $: open = $open_filter == name;

    $: alpha_open = $sort_by == 'alphanumeric';
    $: all_downloads_open = $sort_by == 'all_downloads';
    $: recent_downloads_open = $sort_by == 'recent_downloads';
    $: newly_added_open = $sort_by == 'newly_added';
    $: recently_updated_open = $sort_by == 'recently_updated';

</script>

<div class={open ? 'search filter open' : 'search filter'} use:clickOutside={{ enabled: open, cb: () => open = false }} >

    <button class="filter-box" on:click={() => $open_filter === name ? $open_filter = "" : $open_filter = name}>
        <span class="filter-name">{name}</span>
        <span class="filter-wedge">❮</span>
    </button>

    <div class={open ? 'filter-popup' : 'filter-popup hidden'}>
        <div class="filter-list">
            <label>
                <input type="checkbox" on:click={() => update_sort('alphanumeric')} bind:checked={alpha_open}> Alphabetical
            </label>
            <label>
                <input type="checkbox" on:click={() => update_sort('all_downloads')} bind:checked={all_downloads_open}> Downloads (all-time)
            </label>
            <label>
                <input type="checkbox" on:click={() => update_sort('recent_downloads')} bind:checked={recent_downloads_open}> Downloads (recent)
            </label>
            <label>
                <input type="checkbox" on:click={() => update_sort('newly_added')} bind:checked={newly_added_open}> Newly added
            </label>
            <label>
                <input type="checkbox" on:click={() => update_sort('recently_updated')} bind:checked={recently_updated_open}> Recently updated
            </label>
        </div>
    </div>
</div>
