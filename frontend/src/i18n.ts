import i18n from "i18next"
import {initReactI18next} from "react-i18next"
import en from "./translations/en.json"

const resources = {
  en: {
    translations: en,
  },
}

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    resources,
    lng: "en",
    interpolation: {
      escapeValue: false,
    },
  })

export default i18n
