import { invoke } from '@tauri-apps/api/tauri'

const searchBar = document.querySelectorAll('input');

searchBar.forEach((bar) => {
    bar.addEventListener('keyup', () => {
        invoke('search', {input: bar.value}).then((result) => {
            if (result instanceof Array) {
                result.forEach((track, index) => {
                    let temp = document.getElementById(`${index}`);
                    if (!(null === temp)) {
                        let children = temp.querySelector('p');
                        if (!(children === null)) {
                            children.innerHTML = `${track.name} by ${track.artist}`;
                            children.dataset.id = `${track.id}`;
                            children.dataset.name = `${track.name}`;
                            children.dataset.artist = `${track.artist}`;
                            children.dataset.album = `${track.album}`;
                        }
                    }
                });
            }
        }).catch((_) => console.log('search error'));
    });
});
