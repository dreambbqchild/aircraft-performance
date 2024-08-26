function resizeParent() {
    if(parent && parent.resizeIframes)
        parent.resizeIframes();
}

window.addEventListener('DOMContentLoaded', resizeParent);

function collectAirportData() {
    const data = {};
    const actualMetar = document.getElementById('actual-metar');
    const customMetar = document.getElementById('custom-metar');

    const trySendData = () => {
        if(!data.metar || !data.length)
            return;

        if(parent && parent.loadRunway)
            parent.loadRunway(data);
    }

    actualMetar.addEventListener('change', () => {
        customMetar.value = '';
        data.metar = actualMetar.value;
        trySendData();
    });

    customMetar.addEventListener('blur', () => {
        if(actualMetar)
            actualMetar.selectedIndex = 0;
        
        data.metar = customMetar.value;
        trySendData();
    });

    for(const runway of document.querySelectorAll('input[name="runway"]')) {
        runway.addEventListener('change', () => {
            const row = runway.parentElement.parentElement;
            for(const child of row.childNodes) {
                if(!child.dataset)
                    continue;

                for(let [key, value] of Object.entries(child.dataset)) {
                    if(key === 'length' || key === 'displacedThreshold' || key === 'elevation' || key === 'heading')
                        value = parseInt(value);
                    else if(key === 'isGrass')
                        value = value === 'true';

                    data[key] = value;
                }
            }

            trySendData();
        });
    }
}