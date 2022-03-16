class App extends HTMLElement {
    constructor() {
        super();

        const shadow = this.attachShadow({mode: 'open'});

        const content = document.createElement('span');
        let resp = fetch('/api', {
            method: 'head',
            headers: {},
            mode: 'no-cors'
        }).then(response => {
            for(let entry of response.headers.entries()) {
                console.log(entry);
            }
        });
    }
}

customElements.define('guard-app', App);
