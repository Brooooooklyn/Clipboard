/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class Clipboard {
  constructor()
  setText(text: string): void
  getText(): string
  /** Returns a buffer contains the raw RGBA pixels data */
  getImage(): Buffer
  /** RGBA bytes */
  setImage(width: number, height: number, image: Buffer): void
}
