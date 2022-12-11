function getElementByXpath(path) {
    return document.evaluate(path, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;
}

function step() {
    return getElementByXpath('/html/body/div/section/div[19]/div[2]/div[2]/input[1]');
}

function state() {
    return getElementByXpath('/html/body/div/section/div[19]/div[2]/div[1]').innerText;
}

async function sleep() {
    return new Promise((resolve) => setTimeout(resolve, 100))
}

(async () => {
    const results = [];
    for (var i = 0; i < 256; i++) {
        results.push(this.state())
        step().click();
        await sleep();
    }
    console.log(results)
})()

