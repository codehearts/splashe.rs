class IncrementalMatch {
    constructor(value) {
        this._value = value.toLowerCase();
        this._matches = [0];
    }

    rewindTo(index) {
        if (this._matches.length >= index) {
            while (this._matches.length > index) {
                this._matches.pop();
            }

            return true;
        }
        
        return false;
    }

    matches(character) {
        const index = this._value.indexOf(
            character, this._matches[this._matches.length - 1]);

        if (index === -1) {
            return false;
        } else {
            this._matches.push(index + 1);
        }

        return true;
    }
}

class Splash {
    constructor(link_elements) {
        const container = document.getElementById('splash-zone');

        this._outputElement = container.appendChild(document.createElement('p'));
        this._elementMap = new Map();
        this._matches = new Set();

        for (const element of link_elements) {
            const matcher = new IncrementalMatch(element.innerText);

            this._elementMap.set(matcher, element);
            this._matches.add(matcher);

            element.classList.add('match');
        }

        document.addEventListener('keydown', ev => this.handleKeypress(ev));
    }

    handleKeypress(event) {
        const key = event.key.toLowerCase();
        const previousSelectedElement = this.getFirstMatchElement();

        if (key === 'escape') {
            this._clear();
        } else if (key === 'backspace') {
            this._rewind();
        } else if (key === 'enter') {
            // Follow matched input when user presses "enter"
            if (this._matches.size > 0) {
                window.location.assign(this.getFirstMatchElement().href);
            }
        } else if (key.match(/^[a-zA-Z-_]$/) === null) {
            return;
        } else {
            // Valid input triggers a match progression
            this._matches.forEach(matcher => {
                if (!matcher.matches(key)) {
                    this._elementMap.get(matcher).classList.remove('match');
                    this._matches.delete(matcher);
                }
            });

            // Output the current match pattern
            this._outputElement.innerText += key;
            previousSelectedElement.classList.remove('selected');
            this.getFirstMatchElement().classList.add('selected');
        }
    }

    getFirstMatchElement() {
        return this._elementMap.get(this._matches.values().next().value);
    }

    getQueryLength() {
        return this._outputElement.innerText.length;
    }

    _rewindLinksTo(index) {
        if (index < 1) {
            return;
        }

        this._elementMap.forEach((element, matcher) => {
            if (matcher.rewindTo(index)) {
                this._matches.add(matcher);
                element.classList.add('match');
            }
        });

        this._outputElement.innerText =
            this._outputElement.innerText.slice(
                0, this.getQueryLength() - (this.getQueryLength() - index) - 1);
    }

    _clear() {
        this._rewindLinksTo(1);
    }

    _rewind() {
        this._rewindLinksTo(this.getQueryLength());
    }
}

new Splash(document.querySelectorAll('a'));
