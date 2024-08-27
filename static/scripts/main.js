const arrive = document.getElementById('arrive');
const iframes = document.querySelectorAll("iframe");
const iframeRegistry = {};

for(const iframe of iframes) {
    iframe.height = 0;
    iframeRegistry[iframe.id] = iframe;
}
function resizeIframes() {
    for(const iframe of iframes)
        iframe.height = Math.ceil(iframe.contentWindow.document.body.getBoundingClientRect().bottom);
}

const loadAirport = (value, targetId) => {
    iframeRegistry[targetId].src = `/airport/${value}`;
}

function loadRunway(data) {
    const {id, isTakeoff} = !!iframeRegistry['departing-runway'].src ? {id: 'arrival-runway', isTakeoff: false} : {id: 'departing-runway', isTakeoff: true};
    const {isGrass, metar, heading, elevation} = data;

    iframeRegistry[id].src = `/runway?is_takeoff=${isTakeoff}&is_grass=${isGrass}&metar=${metar}&heading=${heading}&elevation=${elevation}`;
}

function loadPerformance(action, data) {
    const id = action === 'take off' ? 'take-off' : 'landing';
    const {elevation, headwind, standardTemperature, temperature, isGrass} = data; 
    iframeRegistry[id].src = `/aircraft/cessna150j/${id}?elevation_ft=${elevation}&headwind_kts=${headwind}&standard_temperature_f=${standardTemperature}&temperature_f=${temperature}&is_grass=${isGrass}`;
    arrive.style.display = '';
}