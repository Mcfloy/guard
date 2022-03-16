class App extends HTMLElement {
    constructor() {
        super();

        const shadow = this.attachShadow({mode: 'open'});

        const content = document.createElement('span');
        let resp = fetch('http://localhost:3000/api', {
            method: 'head',
            headers: {},
            mode: 'no-cors'
        }).then(response => {
            console.log(response);
            response.headers.forEach(console.log);
            for(let entry of response.headers.entries()) {
                console.log(entry);
            }
        });
    }
}

customElements.define('guard-app', App);
