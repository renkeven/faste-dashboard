// Define DataPoint interface
export interface DataPoint {
	x: number | string; // Datetime
	y: number; // 50POE
	yMin: number; // 90POE
	yMax: number; // 10POE
}

// Define interface with name: string and data: DataPoint[]
export type InputData = {
    name: string;
    data: DataPoint[];
};
