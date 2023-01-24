function urlEncode(obj) {
    return Object
        .keys(obj)
        .map((key) => key + "=" + encodeURIComponent(obj[key]))
        .join("&");
}

function parseJson(res) {
    if (!res.ok) {
        throw `API request failed with response status ${res.status} and text: ${res.statusText}`;
    }

    return res
        .clone()
        .json()
        .catch((_) => res
            .text()
            .then((text) => {
                throw `JSON parsing failed:\n${text}`;
            }
        ));
}

export function GET(endpoint, params={}) {
    return fetch(endpoint + "?" + urlEncode(params))
        .then(parseJson)
        .catch((err) => {
            throw `GET request failed:\n${err}`;
        });
}

export function POST(endpoint, params={}) {
    const args = {
        method: "post",
        headers: { "Content-type": "application/json" },
        body: JSON.stringify(params),
    };
    return fetch(endpoint, args)
        .then(parseJson)
        .catch((err) => {
            throw `POST request failed:\n${err}`;
        });
}