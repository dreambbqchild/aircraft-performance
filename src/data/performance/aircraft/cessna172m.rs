/*
NOTES:
1. Maximum performance technique as specified in Section 4.
2. Prior to takeoff from fields above 3000 feet elevation, the mixture should be leaned to give maximum RPM in a full throttle,
3. Decrease distances 10% for each 9 knots headwind. For operation with tailwinds up to 10 Knots, increase distances by 10% for each 2 knots.
4. Where distance value has been deleted, climb performance after lift-off is less than 150 pm at takeoff speed.
5. For operation on a dry, grass runway, increase distances by 15% of the "ground roll" figure.

1. Maximum performance technique as specified in Section 4.
2. Decrease distances 10% for each 9 knots headwind. For operation with tailwinds up to 10 knots, increase distances by 10% for each 2 knots.
3. For operation on a dry, grass runway, increase distances by 45% of the "ground roll" figure.
*/

use std::cmp::max;

use crate::{data::performance::{distance::Distance, performance_row::PerformanceRow}, math::{FloatingCalcs, Pressure, Velocity}};

const TAKE_OFF_AT_2300_LBS: [[Option<Distance>; 5]; 9] = [
	[Some(Distance(775, 1380)),  Some(Distance(835, 1475)),  Some(Distance(895, 1575)),  Some(Distance(960, 1685)),  Some(Distance(1030, 1795))],
	[Some(Distance(850, 1510)),  Some(Distance(915, 1615)),  Some(Distance(980, 1725)),  Some(Distance(1050, 1845)), Some(Distance(1125, 1970))],
	[Some(Distance(930, 1650)),  Some(Distance(1000, 1770)), Some(Distance(1075, 1895)), Some(Distance(1155, 2030)), Some(Distance(1235, 2170))],
	[Some(Distance(1020, 1815)), Some(Distance(1100, 1945)), Some(Distance(1180, 2085)), Some(Distance(1270, 2235)), Some(Distance(1360, 2395))],
	[Some(Distance(1125, 2000)), Some(Distance(1210, 2145)), Some(Distance(1300, 2305)), Some(Distance(1395, 2475)), Some(Distance(1495, 2655))],
	[Some(Distance(1235, 2210)), Some(Distance(1330, 2375)), Some(Distance(1430, 2555)), Some(Distance(1540, 2750)), Some(Distance(1650, 2960))],
	[Some(Distance(1365, 2450)), Some(Distance(1470, 2640)), Some(Distance(1580, 2850)), Some(Distance(1700, 3070)), None],
	[Some(Distance(1505, 2730)), Some(Distance(1625, 2955)), Some(Distance(1750, 3190)), None,                       None],
	[Some(Distance(1505, 2730)), Some(Distance(1625, 2955)), Some(Distance(1750, 3190)), None,                       None]
];

const TAKE_OFF_AT_2100_LBS: [[Option<Distance>; 5]; 9] = [
	[Some(Distance(630, 1130)),  Some(Distance(680, 1210)),  Some(Distance(725, 1290)),  Some(Distance(780, 1375)),  Some(Distance(835, 1465))],
	[Some(Distance(690, 1235)),  Some(Distance(740, 1320)),  Some(Distance(795, 1405)),  Some(Distance(855, 1500)),  Some(Distance(915, 1600))],
	[Some(Distance(755, 1350)),  Some(Distance(810, 1440)),  Some(Distance(870, 1540)),  Some(Distance(935, 1645)),  Some(Distance(1000, 1755))],
	[Some(Distance(830, 1475)),  Some(Distance(890, 1580)),  Some(Distance(955, 1690)),  Some(Distance(1025, 1805)), Some(Distance(1100, 1930))],
	[Some(Distance(910, 1620)),  Some(Distance(980, 1735)),  Some(Distance(1050, 1860)), Some(Distance(1125, 1990)), Some(Distance(1210, 2130))],
	[Some(Distance(1000, 1780)), Some(Distance(1075, 1910)), Some(Distance(1155, 2050)), Some(Distance(1240, 2195)), Some(Distance(1330, 2355))],
	[Some(Distance(1100, 1965)), Some(Distance(1185, 2115)), Some(Distance(1275, 2270)), Some(Distance(1370, 2435)), Some(Distance(1465, 2615))],
	[Some(Distance(1215, 2180)), Some(Distance(1305, 2345)), Some(Distance(1405, 2520)), Some(Distance(1510, 2715)), Some(Distance(1620, 2920))],
	[Some(Distance(1340, 2425)), Some(Distance(1445, 2615)), Some(Distance(1555, 2815)), Some(Distance(1675, 3040)), Some(Distance(1795, 3280))]
];

const TAKE_OFF_AT_1900_LBS: [[Option<Distance>; 5]; 9] = [
	[Some(Distance(505, 915)),   Some(Distance(540, 975)),   Some(Distance(580, 1035)),  Some(Distance(620, 1105)),  Some(Distance(665, 1175))],
	[Some(Distance(550, 995)),   Some(Distance(590, 1060)),  Some(Distance(635, 1130)),  Some(Distance(680, 1205)),  Some(Distance(725, 1280))],
	[Some(Distance(600, 1085)),  Some(Distance(645, 1155)),  Some(Distance(695, 1230)),  Some(Distance(745, 1315)),  Some(Distance(795, 1400))],
	[Some(Distance(660, 1180)),  Some(Distance(710, 1260)),  Some(Distance(760, 1345)),  Some(Distance(815, 1435)),  Some(Distance(870, 1530))],
	[Some(Distance(725, 1290)),  Some(Distance(775, 1380)),  Some(Distance(835, 1475)),  Some(Distance(895, 1575)),  Some(Distance(955, 1680))],
	[Some(Distance(795, 1415)),  Some(Distance(855, 1515)),  Some(Distance(915, 1620)),  Some(Distance(985, 1735)),  Some(Distance(1055, 1850))],
	[Some(Distance(870, 1555)),  Some(Distance(940, 1670)),  Some(Distance(1010, 1785)), Some(Distance(1080, 1910)), Some(Distance(1160, 2045))],
	[Some(Distance(960, 1715)),  Some(Distance(1035, 1840)), Some(Distance(1110, 1975)), Some(Distance(1195, 2115)), Some(Distance(1280, 2265))],
	[Some(Distance(1060, 1900)), Some(Distance(1140, 2040)), Some(Distance(1225, 2190)), Some(Distance(1320, 2350)), Some(Distance(1415, 2520))]
];

const LANDING_AT_2300_LBS: [[Distance; 5]; 9] = [
	[ Distance(495, 1205), Distance(510, 1235), Distance(530, 1265), Distance(545, 1295), Distance(565, 1330)],
	[ Distance(510, 1235), Distance(530, 1265), Distance(550, 1300), Distance(565, 1330), Distance(585, 1365)],
	[ Distance(530, 1265), Distance(550, 1300), Distance(570, 1335), Distance(590, 1370), Distance(610, 1405)],
	[ Distance(550, 1300), Distance(570, 1335), Distance(590, 1370), Distance(610, 1405), Distance(630, 1440)],
	[ Distance(570, 1335), Distance(590, 1370), Distance(615, 1410), Distance(635, 1445), Distance(655, 1480)],
	[ Distance(590, 1370), Distance(615, 1415), Distance(635, 1450), Distance(655, 1485), Distance(680, 1525)],
	[ Distance(615, 1415), Distance(640, 1455), Distance(660, 1490), Distance(685, 1535), Distance(705, 1570)],
	[ Distance(640, 1455), Distance(660, 1495), Distance(685, 1535), Distance(710, 1575), Distance(730, 1615)],
	[ Distance(665, 1500), Distance(690, 1540), Distance(710, 1580), Distance(735, 1620), Distance(760, 1665)]
];

struct AircraftWeightRowResult<T> {
	presure_altitude_ft: i16,
	lower_temperature_c: i16,
	lower_distance: T,
	upper_temperature_c: i16,
	upper_distance: T
}

#[derive(Clone, Copy)]
pub enum AircraftWeight {
	At2300Lbs = 2300,
	At2100Lbs = 2100,
	At1900Lbs = 1900
}

fn calc_row_column(pressure_altitude: i16, temperature_c: i16) -> Result<(usize, usize), &'static str>{
	let row = max(0, pressure_altitude / 1000) as usize;
	if row > 8  {
		return Err("Altitude out of bounds");
	}

	let column = max(0, temperature_c / 10) as usize;
	if column > 5  {
		return Err("Temperature out of bounds");
	}

	Ok((row, column))
}

impl AircraftWeight {
	pub fn find_takeoff_weight(weight_lbs: i16) -> Result<AircraftWeight, &'static str> {
		if weight_lbs > 2300 {
			Err("Over max weight")
		} else if weight_lbs > 2100 {
			Ok(Self::At2300Lbs)			
		} else if weight_lbs > 1900 {
			Ok(Self::At2100Lbs)
		} else {
			Ok(Self::At1900Lbs)
		}
	}

	fn find_take_off_distance(&self, pressure_altitude: i16, temperature_c: i16) -> Result<AircraftWeightRowResult<Option<Distance>>, &'static str> {
		let result = calc_row_column(pressure_altitude, temperature_c);
		if result.is_err() {
			return Err(result.err().unwrap());
		}

		let (row, column) = result.ok().unwrap();
		if column == 4 && temperature_c % 10 != 0 {
			return Err("Upper column out of bounds");
		}

		let upper_column = if temperature_c % 10 == 0 {
			column
		} else {
			column + 1
		};

		let table = match self {
			AircraftWeight::At2300Lbs => &TAKE_OFF_AT_2300_LBS,
			AircraftWeight::At2100Lbs => &TAKE_OFF_AT_2100_LBS,
			AircraftWeight::At1900Lbs => &TAKE_OFF_AT_1900_LBS
		};

		return Ok(AircraftWeightRowResult {
			presure_altitude_ft: (row * 1000) as i16,
			lower_temperature_c: (column * 10) as i16,
			lower_distance: table[row][column],
			upper_temperature_c: (upper_column * 10) as i16,
			upper_distance: table[row][upper_column]
		});
	}
	
	fn take_off_distance_upper_bound(&self, pressure_altitude_ft: i16, temperature_c: i16) -> Result<AircraftWeightRowResult<Option<Distance>>, &'static str> {
		self.find_take_off_distance(pressure_altitude_ft + 1000, temperature_c)
	}
	
	fn take_off_distance_lower_bound(&self, pressure_altitude_ft: i16, temperature_c: i16) -> Result<AircraftWeightRowResult<Option<Distance>>, &'static str> {
		self.find_take_off_distance(pressure_altitude_ft, temperature_c)
	}

	fn find_landing_distance(&self, pressure_altitude_ft: i16, temperature_c: i16) -> Result<AircraftWeightRowResult<Distance>, &'static str> {
		match self {
			AircraftWeight::At2300Lbs => {
				let result = calc_row_column(pressure_altitude_ft, temperature_c);
				if result.is_err() {
					return Err(result.err().unwrap());
				}

				let (row, column) = result.ok().unwrap();
				if column == 4 && temperature_c % 10 != 0 {
					return Err("Upper column out of bounds");
				}
		
				let upper_column = if temperature_c % 10 == 0 {
					column
				} else {
					column + 1
				};

				return Ok(AircraftWeightRowResult {
					presure_altitude_ft: (row * 1000) as i16,
					lower_temperature_c: (column * 10) as i16,
					lower_distance: LANDING_AT_2300_LBS[row][column],
					upper_temperature_c: (upper_column * 10) as i16,
					upper_distance: LANDING_AT_2300_LBS[row][upper_column]
				});
			},
			_=> Err("Performance not defined")
		}
	}

	fn landing_distance_upper_bound(&self, pressure_altitude_ft: i16, temperature_c: i16) -> Result<AircraftWeightRowResult<Distance>, &'static str> {
		self.find_landing_distance(pressure_altitude_ft + 1000, temperature_c)
	}
	
	fn landing_distance_lower_bound(&self, pressure_altitude_ft: i16, temperature_c: i16) -> Result<AircraftWeightRowResult<Distance>, &'static str> {
		self.find_landing_distance(pressure_altitude_ft, temperature_c)
	}
}

pub struct Corrections {
    pub wind_correction_percentage: f64,
	pub grass_ground_roll_percentage: f64,
    pub distance_corrected_for_wind: Distance,
	pub grass_offset: i16,
    pub distance_corrected_for_grass: Distance
}

pub struct Performance {
	pub pressure_altitude_tween_percentage: f64,
	pub temperature_c_tween_percentage: f64,
	pub lower_temperature_c: i16,
	pub upper_temperature_c: i16,
	pub distance_rows: [PerformanceRow; 3],
	pub corrections: Corrections
}

pub struct Cessna172M {
	pub headwind: Velocity,
	pub pressure_in_hg: f32,
	pub elevation_ft: i16,
	pub pressure_altitude_ft: i16,
	pub temperature_c: i16
}

impl Cessna172M {
	pub fn new(headwind: Velocity, elevation_ft: i16, pressure: Pressure, temperature_c: i16) -> Self {
		let pressure_altitude_ft = pressure.altitude(elevation_ft);
		let pressure_in_hg = pressure.in_hg();

		Cessna172M {
			headwind,
			pressure_in_hg,
			elevation_ft,
			pressure_altitude_ft,
			temperature_c
		}
	}

	fn calc_corrections(&self, distance_at_elevation: Distance, grass_ground_roll_percentage: f64) -> Corrections {
		let headwind_kts = self.headwind.knots();
		let wind_correction_percentage = if headwind_kts > 0 {
			1.0 - (0.1 * (headwind_kts as f64 / 9.0))
		} else {
			1.0 + (0.1 * (headwind_kts as f64 / 2.0))
		};

		let distance_corrected_for_wind = Distance::new_from_f64(
			distance_at_elevation.ground_run() as f64 * wind_correction_percentage, 
			distance_at_elevation.clear_50_ft_obstacle() as f64 * wind_correction_percentage);

		let grass_offset = (distance_corrected_for_wind.ground_run() as f64 * grass_ground_roll_percentage).round() as i16;
		let distance_corrected_for_grass = Distance(distance_corrected_for_wind.ground_run() + grass_offset, distance_corrected_for_wind.clear_50_ft_obstacle() + grass_offset); 

		Corrections {
			wind_correction_percentage,
			grass_ground_roll_percentage,
			distance_corrected_for_wind,
			grass_offset,
			distance_corrected_for_grass
		}
	}

	fn calc_performance(&self, lower_row: AircraftWeightRowResult<Distance>, upper_row: AircraftWeightRowResult<Distance>, grass_ground_roll_percentage: f64) -> Performance {
		let pressure_altitude_tween_percentage = (self.pressure_altitude_ft as f64).percent_i16(lower_row.presure_altitude_ft, upper_row.presure_altitude_ft);
		let temperature_c_tween_percentage = (self.temperature_c as f64).percent_i16(lower_row.lower_temperature_c, lower_row.upper_temperature_c);

		let lower_row_middle_tween = temperature_c_tween_percentage.percent_of_distance(lower_row.lower_distance, lower_row.upper_distance);
		let upper_row_middle_tween = temperature_c_tween_percentage.percent_of_distance(upper_row.lower_distance, upper_row.upper_distance);

		let middle_row_lower_tween = pressure_altitude_tween_percentage.percent_of_distance(lower_row.lower_distance, upper_row.lower_distance);
		let middle_row_upper_tween = pressure_altitude_tween_percentage.percent_of_distance(lower_row.upper_distance, upper_row.upper_distance);
		let distance_at_elevation = pressure_altitude_tween_percentage.percent_of_distance(lower_row_middle_tween, upper_row_middle_tween);

		let distance_rows = [
            PerformanceRow::new_labeled(lower_row.presure_altitude_ft, lower_row.lower_distance, lower_row_middle_tween, lower_row.upper_distance),
            PerformanceRow::new_labeled(self.pressure_altitude_ft, middle_row_lower_tween, distance_at_elevation, middle_row_upper_tween),
            PerformanceRow::new_labeled(upper_row.presure_altitude_ft, upper_row.lower_distance, upper_row_middle_tween, upper_row.upper_distance)
        ];

		Performance {
			pressure_altitude_tween_percentage,
			temperature_c_tween_percentage,
			lower_temperature_c: lower_row.lower_temperature_c,
			upper_temperature_c: upper_row.upper_temperature_c,
			distance_rows,
			corrections: self.calc_corrections(distance_at_elevation, grass_ground_roll_percentage)
		}
	}

	fn convert_to_definate_row_result(&self, row: AircraftWeightRowResult<Option<Distance>>) -> AircraftWeightRowResult<Distance> {
		AircraftWeightRowResult {
			presure_altitude_ft: row.presure_altitude_ft,
			lower_temperature_c: row.lower_temperature_c,
			lower_distance: row.lower_distance.expect("To get the lower distance data"),
			upper_temperature_c: row.upper_temperature_c,
			upper_distance: row.upper_distance.expect("to get the upper distance data")
		}
	}

	pub fn calc_take_off(&self, weight_lbs: i16) -> Performance {
		let takeoff_weight = AircraftWeight::find_takeoff_weight(weight_lbs).expect("To get the takeoff weight");
		let lower_row_optional = takeoff_weight.take_off_distance_lower_bound(self.pressure_altitude_ft, self.temperature_c).expect("To get the lower bound performance numbers");
		let upper_row_optional = takeoff_weight.take_off_distance_upper_bound(self.pressure_altitude_ft, self.temperature_c).expect("To get the upper bound performance numbers");

		let lower_row = self.convert_to_definate_row_result(lower_row_optional);
		let upper_row = self.convert_to_definate_row_result(upper_row_optional);

		self.calc_performance(lower_row, upper_row, 0.15)
	}

	pub fn calc_landing(&self) -> Performance {
		let lower_row = AircraftWeight::At2300Lbs.landing_distance_lower_bound(self.pressure_altitude_ft, self.temperature_c).expect("To get the landing performance");
		let upper_row = AircraftWeight::At2300Lbs.landing_distance_upper_bound(self.pressure_altitude_ft, self.temperature_c).expect("To get the landing performance");

		self.calc_performance(lower_row, upper_row, 0.45)
	}
}