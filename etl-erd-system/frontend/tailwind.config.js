/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        'serif': ['DM Serif Display', 'serif'],
        'sans': ['Inter', 'system-ui', 'sans-serif'],
      },
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        // Modern gradient colors
        gradient: {
          primary: "hsl(var(--gradient-primary))",
          secondary: "hsl(var(--gradient-secondary))",
        },
        glass: {
          bg: "hsl(var(--glass-bg))",
          border: "hsl(var(--glass-border))",
        },
        "accent-blue": "hsl(var(--accent-blue))",
        "accent-purple": "hsl(var(--accent-purple))",
        "accent-pink": "hsl(var(--accent-pink))",
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      backgroundImage: {
        'gradient-primary': 'linear-gradient(135deg, hsl(var(--gradient-primary)), hsl(var(--gradient-secondary)))',
        'gradient-purple-pink': 'linear-gradient(135deg, hsl(var(--accent-purple)), hsl(var(--accent-pink)))',
        'gradient-blue-purple': 'linear-gradient(135deg, hsl(var(--accent-blue)), hsl(var(--accent-purple)))',
        'gradient-radial': 'radial-gradient(ellipse at center, var(--tw-gradient-stops))',
        'glass-gradient': 'linear-gradient(135deg, hsl(var(--glass-bg)), hsl(var(--glass-bg)))',
      },
      boxShadow: {
        'glass': '0 8px 32px 0 rgba(31, 38, 135, 0.37)',
        'modern': '0 10px 40px -10px rgba(0, 0, 0, 0.1)',
        'modern-lg': '0 20px 60px -10px rgba(0, 0, 0, 0.15)',
        'glow': '0 0 20px rgba(147, 51, 234, 0.3)',
        'glow-sm': '0 0 10px rgba(147, 51, 234, 0.2)',
      },
      animation: {
        'gradient-x': 'gradient-x 15s ease infinite',
        'float': 'float 6s ease-in-out infinite',
      },
      keyframes: {
        'gradient-x': {
          '0%, 100%': {
            'background-size': '200% 200%',
            'background-position': 'left center'
          },
          '50%': {
            'background-size': '200% 200%',
            'background-position': 'right center'
          },
        },
        'float': {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-10px)' },
        },
      },
    },
  },
  plugins: [],
  darkMode: ["class"],
}