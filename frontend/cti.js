'use strict';
(() => {
    // some code shamelessly stolen from https://ipv4.games/ until better design idea arrives

    // https://github.com/bryc/code/blob/master/jshash/experimental/cyrb53.js
    const cyrb53 = (str, seed = 0) => {
        let h1 = 0xdeadbeef ^ seed,
            h2 = 0x41c6ce57 ^ seed;
        for (let i = 0, ch; i < str.length; i++) {
            ch = str.charCodeAt(i);
            h1 = Math.imul(h1 ^ ch, 2654435761);
            h2 = Math.imul(h2 ^ ch, 1597334677);
        }
        h1 =
            Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^
            Math.imul(h2 ^ (h2 >>> 13), 3266489909);
        h2 =
            Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^
            Math.imul(h1 ^ (h1 >>> 13), 3266489909);
        return 4294967296 * (2097151 & h2) + (h1 >>> 0);
    }

    const blockColor = (str) => {
        const hue = (((cyrb53(str) & 0xfff) / 0xfff) * 180) | 0;
        return `hsla(${hue}, 90%, 90%, 1)`;
    }

    const nameLink = (name)=> {
        const a = document.createElement('a');
        a.href = `/users.html?name=${encodeURIComponent(name)}`;
        a.innerText = name;
        return a;
    }

    const intoJsonResponse = (response)=> {
        if (response.status === 200) {
            return response.json();
        } else {
            throw new Error("oops, couldn't load the data");
        }
    }

    const ago = (dt, fix = 0) => {
        const formatter = new Intl.RelativeTimeFormat('en', {
            numeric: 'always',
        });
        const diff = (Date.now() - dt) / 1000;
        if (diff < 60) {
            return formatter.format((-diff).toFixed(1), 'seconds');
        }
        if (diff < 60 * 60) {
            return formatter.format((-diff / 60).toFixed(0), 'minutes');
        }
        if (diff < 24 * 60 * 60) {
            return formatter.format((-diff / (60 * 60)).toFixed(0), 'hours');
        }
        if (diff < 31 * 24 * 60 * 60) {
            return formatter.format(
                (-diff / (24 * 60 * 60)).toFixed(0),
                'days'
            );
        }
        if (diff < 366 * 24 * 60 * 60) {
            return formatter.format(
                (-diff / (30 * 24 * 60 * 60)).toFixed(0),
                'months'
            );
        }
        return 'over a year ago';
    }

    const setLastUpdate = (last_updated_at) => {
        document.getElementById(
            'last-updated'
        ).innerText = `(Last updated ${ago(
            Date.parse(last_updated_at + 'Z'),
            1
        )})`;
    }

    const as_ord = (i) => {
        var j = i % 10,
            k = i % 100;
        if (j == 1 && k != 11) {
            return i + 'st';
        }
        if (j == 2 && k != 12) {
            return i + 'nd';
        }
        if (j == 3 && k != 13) {
            return i + 'rd';
        }
        return i + 'th';
    }

    // homepage

    // ip

    const fetchIp = () => {
        fetch('/api/ip')
            .then(intoJsonResponse)
            .then((res) => {
                if (res.v4) {
                    const userIpField = document.getElementById('user-ip');
                    userIpField.innerText = res.ip;
                } else {
                    // todo: note about non-ipv6
                }
            });
    }

    // tops

    window.updateTops = (period) => {
        const url = new URL(location.href);
        url.hash = period;
        location.replace(url);
        location.reload();
    };

    const fetchTops = () => {
        const timeWindow = document.getElementById('rank-period');
        if (location.hash) {
            timeWindow.value = location.hash.slice(1);
        }
        let scoreRoute = {
            hour: '/hour',
            day: '/day',
            week: '/week',
            month: '/month',
            year: '/year',
            all: '/all',
        }[timeWindow.value];

        fetch(`/api/ranks${scoreRoute}`)
            .then(intoJsonResponse)
            .then(({ rankings, last_updated_at }) => {
                const list = document.querySelector('ol');
                rankings.forEach(({ nick, blocks, total_claims }, idx) => {
                    const li = document.createElement('li');
                    li.appendChild(nameLink(nick));
                    li.append(
                        ` with ${total_claims} address${
                            total_claims == 1 ? '' : 'es'
                        }, and ${blocks.length} block${
                            blocks.length > 1 ? 's' : ''
                        } controlled: `
                    );
                    // li.appendChild(document.createElement('br'));
                    li.append(blocks.join(', '));
                    list.appendChild(li);
                });

                const showAllTopsButton =
                    document.getElementById('show-all-tops');
                if (rankings.length > 0) {
                    showAllTopsButton.style.display = '';
                    showAllTopsButton.onclick = function () {
                        showAllTopsButton.style.display = 'none';
                        list.classList.remove('topN');
                    };
                } else {
                    const message = document.getElementById('no-claimers');
                    message.innerText =
                        'No claimers for this period yet. Why not go ahead and capture some IPs?';
                    showAllTopsButton.style.display = 'none';
                }
            });
    }

    // recent

    const fetchRecent = () => {
        fetch('/api/recent')
            .then(intoJsonResponse)
            .then(({ claims }) => {
                const eRecentClaims = document.getElementById('recent-claims');
                const eShowAllRecent =
                    document.getElementById('show-all-recent');
                eRecentClaims.innerHTML = '';

                claims.forEach(({ nick, ip, claimed_at }, idx) => {
                    const div = document.createElement('div');
                    div.classList.add('recent-claim');
                    div.append(`${ip} got captured ${ago(claimed_at * 1000)} by `);
                    div.appendChild(nameLink(nick));
                    if (idx >= 10) {
                        div.style.display = 'none';
                    }
                    eRecentClaims.appendChild(div);
                });
                eShowAllRecent.style.display = '';
                eShowAllRecent.onclick = function () {
                    eShowAllRecent.style.display = 'none';
                    eRecentClaims
                        .querySelectorAll('.recent-claim')
                        .forEach((e) => (e.style.display = ''));
                };
            });
    }

    // block holders

    const fetchBlocks = () => {
        fetch('/api/blocks')
            .then(intoJsonResponse)
            .then(({ blocks, last_updated_at }) => {
                const segmentTemplate = document.getElementById('segment');
                const segmentsContainer = document.getElementById('segments');
                segmentsContainer.innerHTML = '';

                setLastUpdate(last_updated_at);

                for (let i = 0; i < 256; i++) {
                    const el = segmentTemplate.content
                        .cloneNode(true)
                        .querySelector('*');
                    segmentsContainer.appendChild(el);

                    el.querySelector(
                        '.segment-cidr'
                    ).innerText = `${i}.0.0.0/8`;

                    const blockIdx = blocks.findIndex((e) => e.block === i);
                    if (blockIdx >= 0) {
                        const b = blocks[blockIdx];
                        const nickField = el.querySelector('.segment-nick');
                        el.querySelector('.segment-count').innerText = `(${
                            b.claims
                        } address${b.claims == 1 ? '' : 'es'})`;

                        if (b.is_tied) {
                            nickField.innerText = '(tied)';
                            el.classList.add('segment-tied');
                        } else {
                            nickField.appendChild(nameLink(b.nick));
                            el.style.backgroundColor = blockColor(b.nick);
                        }
                    } else {
                        el.classList.add('segment-empty');
                    }
                }
            });
    }

    // user page

    const fetchUser = () => {
        const params = new URLSearchParams(location.search);
        const name = params.get('name');
        const userTitle = document.getElementById('user-name');
        const totalClaims = document.getElementById('total-addresses');
        const list = document.querySelector('ul');
        userTitle.innerText = name;

        fetch(`/api/users?name=${name}`)
            .then(intoJsonResponse)
            .then(({ rankings, last_updated_at }) => {
                setLastUpdate(last_updated_at);

                totalClaims.innerText = rankings.reduce((acc, o) => {
                    return acc + o.claims;
                }, 0);

                for (const e of rankings) {
                    const li = document.createElement('li');
                    const line = `${e.is_tied?'Tied ':''}${as_ord(e.rank)} in block ${e.block}.0.0.0/8 with ${
                        e.claims
                    } address${e.claims == 1 ? '' : 'es'}`;
                    li.innerHTML = line;
                    list.appendChild(li);
                }
            });
    }

    const path = location.pathname;
    if (path === '/' || path === '/index.html') {
        fetchIp();
        fetchRecent();
        fetchTops();
        fetchBlocks();
    } else if (path === '/users.html') {
        fetchUser();
    }
})();
