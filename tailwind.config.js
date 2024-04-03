import { iconsPlugin, getIconCollections } from '@egoist/tailwindcss-icons'
import { fontFamily } from 'tailwindcss/defaultTheme'
import colors from 'tailwindcss/colors'

/** @type {import('tailwindcss').Config} */
export default {
  content: ['src/**/*.{ts,tsx}', 'index.html'],
  darkMode: 'media',
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', ...fontFamily.sans],
        mono: ['JetBrains Mono', ...fontFamily.mono],
      },
      colors: {
        'gray': colors.neutral,
        'background-light': colors.neutral[50],
        'background-dark': colors.neutral[900],
        'foreground-light': colors.neutral[950],
        'foreground-dark': colors.neutral[100],
      },
    },
  },
  plugins: [
    require('@tailwindcss/aspect-ratio'),
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
    require('tailwindcss-animate'),
    require('tailwind-scrollbar')({ nocompatible: true }),
    iconsPlugin({ collections: getIconCollections(['heroicons']) }),
  ],
}
