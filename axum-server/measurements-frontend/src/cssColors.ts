import * as O from 'fp-ts/Option'
import * as TO from 'fp-ts/TaskOption'
import * as T from 'fp-ts/Task'
import { pipe } from 'fp-ts/function'
import type { Mode } from './appState.ts'

const findColorFromCss = (themeCss: string, color: string): O.Option<string> => {
  const varRe = new RegExp(`--${color}\\s*:\\s*([^;]+);?`)
  const m = themeCss.match(varRe)
  const raw = m?.[1]?.trim()
  if (raw) {
    return O.some(raw)
  }
  return O.none
}

const getThemeCss = (mode: Mode): TO.TaskOption<string> =>
  pipe(
    TO.tryCatch(() => Promise.resolve(ui('theme'))),
    TO.chain(
      TO.fromPredicate(
        (theme): theme is IBeerCssTheme =>
          typeof theme === 'object' && theme !== null && mode in theme,
      ),
    ),
    TO.map((theme) => theme[mode]),
  )

const getCssColor = (mode: Mode, color: string): TO.TaskOption<O.Option<string>> =>
  pipe(
    getThemeCss(mode),
    TO.map((themeCss) => findColorFromCss(themeCss, color)),
  )

export const getCssColorOrDefault = (
  mode: Mode,
  color: string,
  fallbackColor: string,
): T.Task<string> =>
  pipe(
    getCssColor(mode, color),
    TO.chain((option) =>
      pipe(
        option,
        O.match(
          () => TO.some(fallbackColor),
          (color) => TO.some(color),
        ),
      ),
    ),
    TO.getOrElse(() => T.of(fallbackColor)),
  )
