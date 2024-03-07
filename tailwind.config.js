/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          moreLighter: '#eaf2f5',
          ligher: '#c2d5df',
          light: '#93a9c1', // Substitua '#suaCorClaraAqui' pelo código hexadecimal da sua cor clara
          DEFAULT: '#70829E', // Substitua '#suaCorPrimariaAqui' pelo código hexadecimal da sua cor primária padrão
          dark: '#4C6285',
          darker: '#2F476C',
          moreDarker: '#4a5665',
          almostBlack: '#2b313b',
          // Substitua '#suaCorEscuraAqui' pelo código hexadecimal da sua cor escura
        },
        secondary: {
          light: '#ffc7c7',
          DEFAULT: '#FF6B6B',
          dark: '#e51d1d',
        },

        // Você pode adicionar mais cores personalizadas aqui conforme necessário
      },
    },
  },
  plugins: [],
};
